# Swedish test social security numbers

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ferenyl/swedish_test_ssn/rust.yml?style=for-the-badge)](https://github.com/ferenyl/swedish_test_ssn/actions/workflows/rust.yml) [![Crates.io Version](https://img.shields.io/crates/v/swedish_test_ssn?style=for-the-badge)](https://crates.io/crates/swedish_test_ssn)

dotnet tools version: [SwedishTestSsn](https://github.com/ferenyl/SwedishTestSsn)

`cargo install swedish_test_ssn`

Get Swedish test ssn from The Swedish Tax Agency (Skatteverket).

These can be used in test environments without risk of breaching GDPR.

Usage: swedish_test_ssn [OPTIONS]

Options:

-p, --pattern [default: .*]

-l, --limit [default: 100]

-o, --offset [default: 0]

-j, --json

-h, --help Print help

-V, --version Print version
