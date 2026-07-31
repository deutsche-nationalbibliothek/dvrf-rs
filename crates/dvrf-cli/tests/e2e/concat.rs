use dvrf::Document;
use predicates::boolean::PredicateBooleanExt;

use crate::prelude::*;

#[test]
fn concat_single_file() -> TestResult {
    let mut cmd = dvrf_cmd();
    let assert = cmd
        .arg("concat")
        .arg(data_dir().join("example1.json"))
        .assert();

    let lhs = Document::from_path(data_dir().join("example1.json"))?;
    let rhs = Document::from_bytes(&assert.get_output().stdout)?;
    assert_eq!(lhs, rhs);

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn concat_multiple_files() -> TestResult {
    let mut cmd = dvrf_cmd();
    let assert = cmd
        .arg("concat")
        .arg(data_dir().join("example1.json"))
        .arg(data_dir().join("example2.json"))
        .assert();

    let doc = Document::from_bytes(&assert.get_output().stdout)?;
    assert_eq!(doc.records().count(), 2);

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());

    Ok(())
}

#[test]
fn concat_pretty_output() -> TestResult {
    let mut cmd = dvrf_cmd();
    let assert = cmd
        .args(["concat", "--pretty"])
        .arg(data_dir().join("example1.json"))
        .assert();

    let lhs = Document::from_path(data_dir().join("example1.json"))?;
    let rhs = Document::from_bytes(&assert.get_output().stdout)?;
    assert_eq!(lhs, rhs);

    assert
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());

    Ok(())
}
