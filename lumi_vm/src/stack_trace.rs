use lumi_lxr::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct TraceFunction {
    name: String,
    class_name: Option<String>,
}

impl TraceFunction {
    pub(crate) fn new(name: &str, class_name: Option<&str>) -> Self {
        Self {
            name: name.to_owned(),
            class_name: class_name.map(|name| name.to_owned()),
        }
    }

    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }

    pub(crate) fn class_name(&self) -> Option<String> {
        self.class_name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Trace {
    span: Span,
    function: Option<TraceFunction>,
}

impl Trace {
    pub(crate) fn new(span: Span, function: Option<TraceFunction>) -> Self {
        Self { span, function }
    }

    pub(crate) fn span(&self) -> Span {
        self.span.clone()
    }

    pub(crate) fn function(&self) -> Option<&TraceFunction> {
        self.function.as_ref()
    }
}

pub type StackTrace = Vec<Trace>;
