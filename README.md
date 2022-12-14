# lan_switch
lan switch register definition

how to use?
in your application, you should define mdio or other function which can read and write switch register,
then use this lib like following:

    fn a_write(dev:u16, reg:u16, val:u16 )->u16 {
      //replace following line with real writing of lan switch
      dev+reg+val
    }
    let ret = mdio_write![a_write, 1, 2, 3];
    println!("ret={:?}",ret);

    fn a_read(dev:u16, reg:u16, ret:&mut i32 )->u16 {
        *ret=0;
        dev+reg
    }
    let mut ret=-1i32;
    let ret = mdio_read_swap![a_read, 1, 2, mv_6390_PORT_0x08, &mut ret];
    println!("ret={:?}",ret);

