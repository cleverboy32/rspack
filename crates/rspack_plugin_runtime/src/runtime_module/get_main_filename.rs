use rspack_core::{
  impl_runtime_module,
  rspack_sources::{BoxSource, RawSource, SourceExt},
  ChunkUkey, Compilation, PathData, RuntimeGlobals, RuntimeModule,
};
use rspack_identifier::Identifier;
use rspack_util::source_map::SourceMapKind;

#[impl_runtime_module]
#[derive(Debug, Eq)]
pub struct GetMainFilenameRuntimeModule {
  chunk: Option<ChunkUkey>,
  id: Identifier,
  global: RuntimeGlobals,
  filename: String,
}

impl GetMainFilenameRuntimeModule {
  pub fn new(content_type: &'static str, global: RuntimeGlobals, filename: String) -> Self {
    Self {
      chunk: None,
      id: Identifier::from(format!("webpack/runtime/get_main_filename/{content_type}")),
      global,
      filename,
      source_map_kind: SourceMapKind::None,
    }
  }
}

impl RuntimeModule for GetMainFilenameRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn generate(&self, compilation: &Compilation) -> BoxSource {
    if let Some(chunk_ukey) = self.chunk {
      let chunk = compilation.chunk_by_ukey.expect_get(&chunk_ukey);
      let filename = compilation.get_path(
        &self.filename.clone().into(),
        PathData::default()
          .chunk(chunk)
          .hash(format!("\" + {}() + \"", RuntimeGlobals::GET_FULL_HASH).as_str())
          .runtime(&chunk.runtime),
      );
      RawSource::from(format!(
        "{} = function () {{
            return \"{}\";
         }};
        ",
        self.global, filename
      ))
      .boxed()
    } else {
      unreachable!("should attach chunk for get_main_filename")
    }
  }

  fn attach(&mut self, chunk: ChunkUkey) {
    self.chunk = Some(chunk);
  }
}
