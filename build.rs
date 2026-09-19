#[cfg(target_os = "windows")]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set_manifest(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*" />
    </dependentAssembly>
  </dependency>
</assembly>"#);
    let version = env!("CARGO_PKG_VERSION");
    let file_version = format!("{version}.0");
    res.set("CompanyName", "vorojar");
    res.set("FileDescription", "MD Preview");
    res.set("FileVersion", &file_version);
    res.set("LegalCopyright", "Copyright (c) vorojar");
    res.set("ProductName", "MD Preview");
    res.set("ProductVersion", version);
    res.compile().expect("failed to compile Windows resources");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
