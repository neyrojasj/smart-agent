use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use tempfile::TempDir;

// @verifies REQ-048 v3 74c0bba3
#[test]
fn init_installs_skills_from_binary_without_agent_tools_directory() {
    let dir = TempDir::new().expect("create temp dir");
    // .claude/ triggers Claude provider detection without any stdin prompt
    fs::create_dir_all(dir.path().join(".claude")).expect("create .claude dir");

    assert!(
        !dir.path().join("agent-tools").exists(),
        "precondition: no agent-tools/ directory must exist"
    );

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let skills_dir = dir.path().join(".claude/skills");
    assert!(
        skills_dir.is_dir(),
        "expected .claude/skills/ to be created by init"
    );

    let skill_count = fs::read_dir(&skills_dir)
        .expect("read .claude/skills")
        .flatten()
        .filter(|e| e.path().is_dir())
        .count();

    assert!(
        skill_count > 0,
        "expected at least one skill installed from the embedded registry"
    );
}

// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_creates_docs_prds_directory() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join("docs/prds").is_dir(),
        "expected docs/prds/ to be created"
    );
}

#[test]
fn init_creates_docs_decisions_directory() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join("docs/decisions").is_dir(),
        "expected docs/decisions/ to be created"
    );
}

// @verifies REQ-035 v2 1e646999
#[test]
fn init_creates_default_weftignore() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let weftignore = dir.path().join(".weftignore");
    let src = fs::read_to_string(&weftignore).expect("read .weftignore");
    assert!(
        src.contains(".scratch"),
        "expected .scratch entry, got:\n{src}"
    );
    assert!(src.contains("logs"), "expected logs entry, got:\n{src}");
}

// @verifies REQ-035 v2 1e646999
#[test]
fn init_does_not_overwrite_existing_weftignore() {
    let dir = TempDir::new().expect("create temp dir");
    let weftignore = dir.path().join(".weftignore");
    fs::write(&weftignore, "custom\n").expect("write fixture .weftignore");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let src = fs::read_to_string(&weftignore).unwrap();
    assert_eq!(
        src, "custom\n",
        "expected init to leave existing .weftignore untouched, got:\n{src}"
    );
}

#[test]
fn init_is_idempotent_on_existing_project() {
    let dir = TempDir::new().expect("create temp dir");

    // Run init twice — should not fail the second time
    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();
}

// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_second_run_does_not_overwrite_existing_prd_record() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let req_path = dir.path().join("docs/prds/REQ-001.toml");
    fs::write(&req_path, "id = \"REQ-001\"\n").expect("write fixture record");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let src = fs::read_to_string(&req_path).unwrap();
    assert_eq!(
        src, "id = \"REQ-001\"\n",
        "expected init to leave existing records untouched, got:\n{src}"
    );
}

// @verifies REQ-050 v2 b7e04277
// @verifies REQ-051 v2 16a337fc
#[test]
fn agent_tools_skills_directories_exist() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let skills_dir = workspace.join("agent-tools/skills");

    for skill in &[
        "weft",
        "grill-with-docs",
        "architecture",
        "issue-tracker",
        "domain",
        "triage",
    ] {
        let skill_dir = skills_dir.join(skill);
        assert!(
            skill_dir.is_dir(),
            "expected agent-tools/skills/{skill}/ to exist"
        );
        assert!(
            skill_dir.join("SKILL.md").is_file(),
            "expected agent-tools/skills/{skill}/SKILL.md to exist"
        );
    }
}

// @verifies REQ-049 v2 a5273df5
#[test]
fn init_creates_context_md() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join(".claude")).expect("create .claude dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let context_path = dir.path().join("CONTEXT.md");
    assert!(context_path.exists(), "expected CONTEXT.md to be created");

    let src = fs::read_to_string(&context_path).expect("read CONTEXT.md");
    assert!(src.contains("weft"), "expected weft skill reference");
    assert!(src.contains(".scratch"), "expected .scratch/ issue tracker reference");
    assert!(src.contains("docs/adr"), "expected docs/adr/ reference");
    assert!(src.contains("docs/decisions"), "expected docs/decisions/ reference");
}

// @verifies REQ-049 v2 a5273df5
#[test]
fn init_does_not_overwrite_existing_context_md() {
    let dir = TempDir::new().expect("create temp dir");
    let context_path = dir.path().join("CONTEXT.md");
    fs::write(&context_path, "# My custom context\n").expect("write fixture CONTEXT.md");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let src = fs::read_to_string(&context_path).expect("read CONTEXT.md");
    assert_eq!(
        src, "# My custom context\n",
        "expected init to leave existing CONTEXT.md untouched, got:\n{src}"
    );
}

// @verifies REQ-049 v2 a5273df5
#[test]
fn init_context_md_lists_installed_skills() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join(".claude")).expect("create .claude dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let src = fs::read_to_string(dir.path().join("CONTEXT.md")).expect("read CONTEXT.md");
    assert!(
        src.contains("weft"),
        "expected CONTEXT.md to list the weft skill, got:\n{src}"
    );
}

// @verifies REQ-051 v2 16a337fc
#[test]
fn grill_with_docs_skill_includes_supporting_files() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let grill_dir = workspace.join("agent-tools/skills/grill-with-docs");

    assert!(
        grill_dir.join("CONTEXT-FORMAT.md").is_file(),
        "expected agent-tools/skills/grill-with-docs/CONTEXT-FORMAT.md"
    );
    assert!(
        grill_dir.join("ADR-FORMAT.md").is_file(),
        "expected agent-tools/skills/grill-with-docs/ADR-FORMAT.md"
    );
}

// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_creates_docs_adr_directory() {
    let dir = TempDir::new().expect("create temp dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    assert!(
        dir.path().join("docs/adr").is_dir(),
        "expected docs/adr/ to be created"
    );
}

// @verifies REQ-048 v3 74c0bba3
// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_installs_scripts_for_claude() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join(".claude")).expect("create .claude dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let script = dir.path().join(".claude/scripts/afk-ralph.sh");
    assert!(
        script.is_file(),
        "expected .claude/scripts/afk-ralph.sh to be installed"
    );
    let content = fs::read_to_string(&script).expect("read afk-ralph.sh");
    assert!(
        content.contains("afk-claude"),
        "expected Claude-specific script content"
    );
}

// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_installs_claude_md_instruction_file() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join(".claude")).expect("create .claude dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let claude_md = dir.path().join("CLAUDE.md");
    assert!(
        claude_md.is_file(),
        "expected CLAUDE.md to be created for Claude provider"
    );
    let content = fs::read_to_string(&claude_md).expect("read CLAUDE.md");
    assert!(
        content.contains("CONTEXT.md"),
        "expected CLAUDE.md to reference CONTEXT.md"
    );
}

// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_does_not_overwrite_existing_claude_md() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join(".claude")).expect("create .claude dir");
    let claude_md = dir.path().join("CLAUDE.md");
    fs::write(&claude_md, "# My custom CLAUDE.md\n").expect("write fixture CLAUDE.md");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let content = fs::read_to_string(&claude_md).expect("read CLAUDE.md");
    assert_eq!(
        content, "# My custom CLAUDE.md\n",
        "expected init to leave existing CLAUDE.md untouched"
    );
}

// @verifies REQ-048 v3 74c0bba3
// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_installs_scripts_for_copilot() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join(".github/copilot")).expect("create .github/copilot dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let script = dir.path().join(".github/scripts/afk-ralph.sh");
    assert!(
        script.is_file(),
        "expected .github/scripts/afk-ralph.sh to be installed"
    );
    let content = fs::read_to_string(&script).expect("read afk-ralph.sh");
    assert!(
        content.contains("afk-copilot"),
        "expected Copilot-specific script content"
    );
}

// @verifies REQ-013 v7 fd2545ef
#[test]
fn init_installs_copilot_instruction_file() {
    let dir = TempDir::new().expect("create temp dir");
    fs::create_dir_all(dir.path().join(".github/copilot")).expect("create .github/copilot dir");

    Command::cargo_bin("weft")
        .unwrap()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success();

    let instructions = dir.path().join(".github/copilot-instructions.md");
    assert!(
        instructions.is_file(),
        "expected .github/copilot-instructions.md to be created for Copilot provider"
    );
    let content = fs::read_to_string(&instructions).expect("read copilot-instructions.md");
    assert!(
        content.contains("CONTEXT.md"),
        "expected copilot-instructions.md to reference CONTEXT.md"
    );
}
