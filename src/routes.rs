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