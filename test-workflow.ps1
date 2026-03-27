param(
    [string]$Archive = "workflow-test.rat",
    [switch]$Clean
)

$ErrorActionPreference = "Stop"

function Invoke-Step {
    param(
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][scriptblock]$Action
    )

    Write-Host ""
    Write-Host "==> $Name" -ForegroundColor Cyan
    & $Action
}

function Assert-True {
    param(
        [Parameter(Mandatory = $true)][bool]$Condition,
        [Parameter(Mandatory = $true)][string]$Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

Push-Location $PSScriptRoot
try {
    $testFiles = @(
        @{ Input = "1.png"; Compression = "default"; Extracted = "1.extracted.png" },
        @{ Input = "2.png"; Compression = "default"; Extracted = "2.extracted.png" },
        @{ Input = "3.png"; Compression = "default"; Extracted = "3.extracted.png" }
    )

    foreach ($entry in $testFiles) {
        Assert-True -Condition (Test-Path $entry.Input) -Message "Input file not found: $($entry.Input)"
    }

    $exePath = Join-Path $PSScriptRoot "target\debug\file_rat.exe"

    if ($Clean) {
        Invoke-Step -Name "Cleaning previous artifacts" -Action {
            if (Test-Path $Archive) { Remove-Item $Archive -Force }

            foreach ($entry in $testFiles) {
                if (Test-Path $entry.Extracted) {
                    Remove-Item $entry.Extracted -Force
                }
            }
        }
    }

    Invoke-Step -Name "Building project" -Action {
        cargo build | Out-Host
        Assert-True -Condition ($LASTEXITCODE -eq 0) -Message "cargo build failed"
        Assert-True -Condition (Test-Path $exePath) -Message "Executable not found after build: $exePath"
    }

    Invoke-Step -Name "Adding 3 files with different compression levels" -Action {
        foreach ($entry in $testFiles) {
            Write-Host ("ADD  -> {0} | compression={1}" -f $entry.Input, $entry.Compression) -ForegroundColor DarkCyan

            if (Test-Path $Archive) {
                & $exePath add $Archive $entry.Input --compression $entry.Compression --meta workflow=ps1 --meta stage=add | Out-Host
            }
            else {
                "y" | & $exePath add $Archive $entry.Input --compression $entry.Compression --meta workflow=ps1 --meta stage=add | Out-Host
            }

            Assert-True -Condition ($LASTEXITCODE -eq 0) -Message "add command failed for $($entry.Input)"
        }
    }

    Invoke-Step -Name "Listing archive" -Action {
        $listOutput = & $exePath list $Archive
        $listOutput | Out-Host
        Assert-True -Condition ($LASTEXITCODE -eq 0) -Message "list command failed"
        $listText = ($listOutput | Out-String)

        foreach ($entry in $testFiles) {
            $inputName = [System.IO.Path]::GetFileName($entry.Input)
            $containsInput = $listText -match [regex]::Escape($inputName)
            Assert-True -Condition $containsInput -Message "list output does not contain $inputName"
        }
    }

    Invoke-Step -Name "Extracting and comparing file content" -Action {
        foreach ($entry in $testFiles) {
            if (Test-Path $entry.Extracted) {
                Remove-Item $entry.Extracted -Force
            }

            $inputName = [System.IO.Path]::GetFileName($entry.Input)
            Write-Host ("EXTRACT -> {0} | expected compression={1}" -f $inputName, $entry.Compression) -ForegroundColor DarkYellow
            & $exePath extract $Archive $inputName $entry.Extracted | Out-Host
            Assert-True -Condition ($LASTEXITCODE -eq 0) -Message "extract command failed for $inputName"
            Assert-True -Condition (Test-Path $entry.Extracted) -Message "extracted file not created: $($entry.Extracted)"

            $srcHash = (Get-FileHash -Algorithm SHA256 -Path $entry.Input).Hash
            $dstHash = (Get-FileHash -Algorithm SHA256 -Path $entry.Extracted).Hash
            Assert-True -Condition ($srcHash -eq $dstHash) -Message "content mismatch after extraction for $inputName"
        }
    }

    $archiveSizeBytes = (Get-Item $Archive).Length

    Write-Host ""
    Write-Host "Workflow test completed successfully." -ForegroundColor Green
    Write-Host ("Archive size: {0} bytes" -f $archiveSizeBytes) -ForegroundColor Green
}
catch {
    Write-Host ""
    Write-Host "Workflow test failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
finally {
    Pop-Location
}
