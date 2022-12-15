use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::Method;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct IndexTemplate;

#[derive(TemplateOnce)]
#[template(path = "privacy.stpl")]
struct PrivacyTemplate;

#[derive(TemplateOnce)]
#[template(path = "terms.stpl")]
struct TermsTemplate;

pub async fn default(req: HttpRequest) -> HttpResponse {
  if req.method() != &Method::GET {
    return HttpResponse::NotFound().finish();
  }

  HttpResponse::NotFound().body("where r u going")
}

fn render<T: TemplateOnce>(req: HttpRequest, template: T) -> HttpResponse {
  if req.method() != &Method::GET {
    return HttpResponse::MethodNotAllowed().finish();
  }

  if let Ok(body) = template.render_once() {
    HttpResponse::Ok()
      .content_type(ContentType::html())
      .body(body)
  } else {
    HttpResponse::InternalServerError().body("something went wrong")
  }
}

pub async fn index(req: HttpRequest) -> HttpResponse {
  render(req, IndexTemplate)
}

pub async fn privacy(req: HttpRequest) -> HttpResponse {
  render(req, PrivacyTemplate)
}

pub async fn terms(req: HttpRequest) -> HttpResponse {
  render(req, TermsTemplate)
}