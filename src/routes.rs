use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::header::{self, ContentType};
use actix_web::http::{Method, StatusCode};
use sailfish::TemplateOnce;

const INVITE_URL: &str = "https://discord.com/oauth2/authorize?client_id=706855160453791784&scope=bot%20applications.commands&permissions=274914642944";
const SUPPORT_URL: &str = "https://discord.com/invite/Nxq6FbFYqp";

#[derive(TemplateOnce)]
#[template(path = "error.stpl")]
struct ErrorTemplate {
  status_code: StatusCode,
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct IndexTemplate;

#[derive(TemplateOnce)]
#[template(path = "privacy.stpl")]
struct PrivacyTemplate;

#[derive(TemplateOnce)]
#[template(path = "terms.stpl")]
struct TermsTemplate;

#[derive(TemplateOnce)]
#[template(path = "premium.stpl")]
struct PremiumTemplate;

#[derive(TemplateOnce)]
#[template(path = "commands.stpl")]
struct CommandsTemplate<'t> {
  commands: &'t [Command<'t>],
}

fn render<T: TemplateOnce>(req: HttpRequest, template: T, status_code: StatusCode) -> HttpResponse {
  if req.method() != &Method::GET {
    return HttpResponse::MethodNotAllowed().finish();
  }

  HttpResponse::build(status_code)
    .content_type(ContentType::html())
    .body(template.render_once().unwrap())
}

fn render_error(req: HttpRequest, status_code: StatusCode) -> HttpResponse {
  render(req, ErrorTemplate {
    status_code,
  }, status_code)
}

fn render_ok<T: TemplateOnce>(req: HttpRequest, template: T) -> HttpResponse {
  render(req, template, StatusCode::OK)
}

fn redirect(req: HttpRequest, url: &str) -> HttpResponse {
  if req.method() != &Method::GET {
    HttpResponse::MethodNotAllowed().finish()
  } else {
    HttpResponse::Found().insert_header((header::LOCATION, url)).finish()
  }
}

pub async fn default(req: HttpRequest) -> HttpResponse {
  if req.method() != &Method::GET {
    HttpResponse::NotFound().finish()
  } else {
    render_error(req, StatusCode::NOT_FOUND)
  }
}

pub async fn index(req: HttpRequest) -> HttpResponse {
  render_ok(req, IndexTemplate)
}

pub async fn privacy(req: HttpRequest) -> HttpResponse {
  render_ok(req, PrivacyTemplate)
}

pub async fn terms(req: HttpRequest) -> HttpResponse {
  render_ok(req, TermsTemplate)
}

pub async fn invite(req: HttpRequest) -> HttpResponse {
  redirect(req, INVITE_URL)
}

pub async fn support(req: HttpRequest) -> HttpResponse {
  redirect(req, SUPPORT_URL)
}

pub async fn premium(req: HttpRequest) -> HttpResponse {
  render_ok(req, PremiumTemplate)
}

struct Command<'c> {
  name: &'c str,
  description: &'c str,
  speech_example: Option<&'c str>,
  dms: bool,
  options: &'c [&'c str],
}

impl<'c> Command<'c> {
  const fn new(name: &'c str, description: &'c str, speech_example: Option<&'c str>, dms: bool) -> Self {
    Self::new_with_options(name, description, speech_example, dms, &[])
  }

  const fn new_with_options(name: &'c str, description: &'c str, speech_example: Option<&'c str>, dms: bool, options: &'c [&str]) -> Self {
    Self {
      name,
      description,
      speech_example,
      dms,
      options,
    }
  }
}

pub async fn commands(req: HttpRequest) -> HttpResponse {
  const COMMANDS: [Command; 20] = [
    Command::new_with_options("fast-forward", "Forward the player by the specified amount of seconds.", None, false, &["seconds"]),
    Command::new("help", "View useful information.", None, true),
    Command::new("join", "Connect me to your voice channel.", None, false),
    Command::new("leave", "Disconnect me from your voice channel.", Some("disconnect"), false),
    Command::new("loop", "Toggle track loop.", None, false),
    Command::new("pause", "Pause the audio playback.", Some("pause"), false),
    Command::new_with_options("play", "Play a track or add it to the queue.", Some("play Never Gonna Give You Up"), false, &["what"]),
    Command::new("queue clear", "Remove all tracks from the queue.", None, false),
    Command::new("queue list", "View information about the queued tracks.", None, false),
    Command::new_with_options("queue remove", "Remove a specific track from the queue.", Some("remove the first track"), false, &["index"]),
    Command::new("queue shuffle", "Randomize the order of tracks in queue.", None, false),
    Command::new("resume", "Resume the audio playback.", Some("resume"), false),
    Command::new_with_options("rewind", "Rewind the player by the specified amount of seconds.", None, false, &["seconds"]),
    Command::new_with_options("skip", "Skip to the next track or to the specified track index.", Some("skip"), false, &["to"]),
    Command::new("stop", "End the audio playback and clear the queue.", Some("stop"), false),
    Command::new("track", "View information about the current track.", None, false),
    Command::new("user rank", "View your rank card.", None, true),
    Command::new("user settings", "View and modify your settings.", None, true),
    Command::new("volume get", "View the volume.", Some("what's the volume"), false),
    Command::new_with_options("volume set", "Modify the volume.", Some("set the volume at 75"), false, &["value"]),
  ];

  render_ok(req, CommandsTemplate {
    commands: &COMMANDS,
  })
}