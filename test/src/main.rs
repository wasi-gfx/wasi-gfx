#[cfg(test)]
mod tests {
    #[test]
    fn frame_buffer_parses() {
        package_parses("frame-buffer");
    }

    #[test]
    fn surface_parses() {
        package_parses("surface");
    }

    fn package_parses(package: &str) {
        let mut resolve = wit_parser::Resolve::new();
        resolve
            .push_dir(&format!("../packages/{package}/wit"))
            .expect("failed to parse");
    }
}
