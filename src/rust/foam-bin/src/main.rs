use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
  #[derive(Debug, StructOpt, strum_macros::EnumString)]
  enum Command {
    Open,
    New { path: std::path::PathBuf },
  }
  impl Default for Command {
    fn default() -> Self { Command::Open }
  }
  #[derive(Debug, StructOpt)]
  #[structopt(
    name = "foam",
    about = "Command-line tool to simplify working with `foam` workspaces"
  )]
  struct App {
    #[structopt(subcommand)]
    cmd: Option<Command>,
  }

  impl App {
    fn run(self) -> anyhow::Result<()> {
      match self.cmd.unwrap_or_default() {
        Command::Open => {
          if std::path::Path::new(".vscode").join("foam.json").exists() {
            duct::cmd!("code", ".").run()?;
          } else {
            return Err(anyhow::anyhow!("This is not a Foam workspace!"));
          }
        }
        Command::New { path } => {
          duct::cmd!("npx", "degit", "https://github.com/foambubble/foam-template", path).run()?;
        }
      }

      Ok(())
    }
  }

  let app = App::from_args();

  app.run()
}
