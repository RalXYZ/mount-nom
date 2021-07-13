extern crate mount_nom;

fn main() {
    let (_, mount) =
        mount_nom::p("blk /mnt FAT32 options,a,b=c,d\\040e 0 0")
            .unwrap();

    println!("{:?}", mount);
}
