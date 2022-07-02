mod mtktextureloader;

#[link(name="MetalKit",kind="framework")]
extern "C" {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
