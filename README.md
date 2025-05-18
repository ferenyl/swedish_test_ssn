# Swedish test social security numbers

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ferenyl/swedish_test_ssn/rust.yml?style=for-the-badge)](https://github.com/ferenyl/swedish_test_ssn/actions/workflows/rust.yml) [![Crates.io Version](https://img.shields.io/crates/v/swedish_test_ssn?style=for-the-badge)](https://crates.io/crates/swedish_test_ssn)

dotnet tools version: [SwedishTestSsn](https://github.com/ferenyl/SwedishTestSsn)

`cargo install swedish_test_ssn`

```bash
Get swedish test ssn from The Swedish Tax Agency (Skatteverket). Great for testing without risking breaching GDPR

Usage: swedish_test_ssn [OPTIONS]

Options:
  -p, --pattern <PATTERN>  Pattern for ssn. Regular expressions can be used [default: .*]
  -l, --limit <LIMIT>      Limit the number of items returned [default: 100]
  -o, --offset <OFFSET>    Number of items to skip [default: 0]
  -j, --json               Return as json array
  -h, --help               Print help
  -V, --version            Print version
```
