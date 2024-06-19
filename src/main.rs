mod structs;
mod metatest;

fn main() {
    //let rfile= new Rfile<Metadata.class>(fileRef);

    /*
    > add to end of data -> binrw file content
    > add to end of hadr -> (start>end + file name + metadata in json + unique id)


    file format :

    [--@DATA@--|headers]


    pre-cond to an update :
        - the disk needs filesize in worst case of room + headers
        - rights 700 on file
        - rat file is not in EOF

    technical specs :
        - the Metadata motherclass must have a serial id autocalculated
        - the rat processor can be made
                - serial ignorant to disable serial checks
                - high/low compression level
                - encrypted headers


    */
}
