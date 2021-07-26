# Edge Proxy Mode Formatter

This is a simple [Rust](https://www.rust-lang.org/) CLI app for generating valid JSON
in accordance to Microsoft Edge Group Policy options for
Proxy Settings.

See <https://docs.microsoft.com/en-us/deployedge/microsoft-edge-policies#proxysettings>
for the full explanation of proxy options for Edge GPOs.

## Usage

```text
USAGE:
   edge-proxy-json-gen.exe [FLAGS] [OPTIONS] <mode>

   FLAGS:
        -h, --help       Prints help information
        -f, --pretty     Emit formatted 'pretty' JSON.
        -V, --version    Prints version information
   OPTIONS:
        -u, --pac-url <pac-url>                  URL containing proxy .pac file.
        -b, --bypass-list <proxy-bypass-list>    List of IP addresses to bypass proxy.
        -s, --server <proxy-server>              URI of proxy server including port.
   ARGS:
        <mode>    Proxy Mode.
```

## License

MIT

## Example

* Pretty-print JSON for a direct connection with no proxy.

  `edge-proxy-json-gen.exe -f "direct"`

   This will emit the following:

   ```json
   {
     "proxy_mode": "direct",
     "proxy_pac_url": "",
     "proxy_server": "",
     "proxy_bypass_list": ""
   }
   ```

* Minified JSON for a proxy server with a bypass specified.

   `edge-proxy-json-gen.exe "fixed-servers" -s "192.168.1.1:8080" -b "192.168.1.2"`

   emits the following:

   ```json
   {"proxy_mode":"fixed_servers","proxy_pac_url":"","proxy_server":"192.168.1.1:8080","proxy_bypass_list":"192.168.1.2"}
   ```
