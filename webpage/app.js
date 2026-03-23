/* ======================================================
   Translations
   ====================================================== */
const t = {
  en: {
    /* page */
    'page.title': 'leptos-content-collection — Astro-style content collections for Rust',
    'page.desc':  'A zero-boilerplate Rust crate for loading typed Markdown content collections. Works at build time or runtime, framework-agnostic.',

    /* nav */
    'menu.toggle':         'Menu',
    'nav.group.start':     'Getting Started',
    'nav.overview':        'Overview',
    'nav.features':        'Features',
    'nav.content-format':  'Content Format',
    'nav.markdown-styling':'Markdown Styling',
    'nav.group.usage':     'Usage',
    'nav.buildtime':       'Buildtime',
    'nav.default-badge':   'default',
    'nav.bt-install':      '1. Installation',
    'nav.bt-buildrs':      '2. build.rs',
    'nav.bt-schema':       '3. Schema & load',
    'nav.ssr':             'SSR / Runtime',
    'nav.leptos':          'Leptos Integration',
    'nav.group.ref':       'Reference',
    'nav.group.meta':      'Meta',
    'nav.dependencies':    'Dependencies',
    'nav.freya':           'Freya Integration',

    /* hero */
    'hero.tagline': 'Astro-style content collections for Rust — define a schema struct, point it at a directory of Markdown files, get back a fully typed collection.',
    'badge.agnostic': 'framework-agnostic',

    /* overview */
    'overview.title': 'Overview',
    'overview.p1': '<code>leptos-content-collection</code> brings the content collections concept popularised by <a href="https://docs.astro.build/en/guides/content-collections/" target="_blank">Astro</a> to the Rust ecosystem. Each directory of <code>.md</code> files becomes a typed collection: your frontmatter is deserialised into a Rust struct at compile time or load time, and the Markdown body can be rendered to HTML on demand.',
    'overview.callout': 'Despite the name, this crate has <strong>no dependency on Leptos</strong>. It works equally well with Axum, Actix-web, static site generators, CLIs, or any other Rust project.',

    /* features */
    'features.title': 'Features',
    'feat.default': 'default',
    'feat.buildtime.desc': 'Content files are parsed during <code>cargo build</code> and embedded directly into the binary. No filesystem access at runtime — works in SSR servers and WASM bundles alike.',
    'feat.ssr.desc': 'Reads <code>.md</code> files from the filesystem at request time via <code>Collection::load()</code>. Useful when you want to update content without recompiling.',
    'features.intro': 'Both features can be active simultaneously. The table below summarises what each one enables:',
    'features.th.feature': 'Feature',
    'features.th.default': 'Default',
    'features.th.methods': 'Method unlocked',

    /* content format */
    'format.title': 'Content Format',
    'format.p': 'Every <code>.md</code> file must start with a YAML frontmatter block delimited by <code>---</code>. The <strong>slug</strong> for each entry is the filename without the <code>.md</code> extension.',
    'format.th.part': 'Part',
    'format.th.desc': 'Description',
    'format.row.fm.label': 'First <code>---</code> block',
    'format.row.fm.desc': 'YAML frontmatter. Deserialised into your schema struct.',
    'format.row.body.label': 'Body',
    'format.row.body.desc': 'Raw Markdown. Accessible via <code>entry.body</code> and renderable with <code>entry.render()</code>.',
    'format.row.slug.desc': 'Filename without <code>.md</code>, e.g. <code>"hello-world"</code>.',
    'mdstyle.title': 'Styling rendered Markdown',
    'mdstyle.p1': '<code>entry.render()</code> returns an HTML string. Render it inside a wrapper element (for example, <code>.md-content</code>) and scope your typography styles to that wrapper.',
    'mdstyle.note': 'If your content can come from untrusted sources, sanitize the generated HTML before injecting it in the browser.',
    'copy.btn': 'copy',

    /* buildtime */
    'bt.title': 'Buildtime',
    'bt.p': 'Content is processed during <code>cargo build</code> and embedded in the binary as static string literals. No <code>std::fs</code> calls happen at runtime, so the same binary works for both SSR servers and WASM bundles.',
    'bt.step1.title': 'Add the dependency',
    'bt.step2.title': 'Create a <code>build.rs</code>',
    'bt.step2.desc': '<code>codegen::generate(dir, name)</code> scans <code>dir</code> for <code>.md</code> files, embeds their content and writes <code>$OUT_DIR/{name}_collection.rs</code>. It also emits <code>cargo:rerun-if-changed</code> so the build re-runs whenever a content file is added, edited, or removed.',
    'bt.step3.title': 'Define your schema and load',

    /* ssr */
    'ssr.title': 'SSR / Runtime',
    'ssr.p': 'Enable the <code>ssr</code> feature to read files from the filesystem at request time. Useful when you want to edit content without rebuilding.',
    'ssr.callout': '<code>Collection::load()</code> uses <code>std::fs</code> and will not compile for <code>wasm32</code> targets. In a Leptos project, call it only inside <code>#[server]</code> functions or Axum handlers.',

    /* leptos */
    'nav.yew':    'Yew Integration',
    'nav.dioxus': 'Dioxus Integration',
    'leptos.title': 'Leptos Integration',
    'leptos.p': 'A typical Leptos + Axum setup uses <strong>both features</strong>: <code>buildtime</code> so the WASM bundle can access data without a server round-trip, and <code>ssr</code> (optionally) inside <code>#[server]</code> functions for hot-reloadable content in development.',
    'leptos.sub3.title': 'app.rs — using buildtime data (works in SSR + WASM)',
    'leptos.callout': '<code>codegen::generate</code> is gated with <code>cfg(not(target_arch = "wasm32"))</code> inside the crate, so it is never compiled into the WASM bundle — only into <code>build.rs</code> (which runs on the host).',

    /* yew */
    'yew.title': 'Yew Integration',
    'yew.p': 'Yew compiles to WASM, so only the <code>buildtime</code> feature works — content is embedded at compile time and requires no filesystem access at runtime.',
    'yew.sub3.title': 'app.rs — Yew component',
    'yew.callout': 'The <code>ssr</code> feature uses <code>std::fs</code> and will not compile for <code>wasm32</code> targets. Stick with <code>buildtime</code> for pure-WASM Yew apps.',

    /* dioxus */
    'dioxus.title': 'Dioxus Integration',
    'dioxus.p': 'Dioxus supports multiple renderers (web, desktop, mobile). For WASM targets use <code>buildtime</code>; for desktop / native targets both features work.',
    'dioxus.sub.wasm.title': 'Web (WASM) — buildtime',
    'dioxus.sub.desktop.title': 'Desktop — ssr feature (runtime filesystem)',
    'dioxus.callout': 'Freya (a native GUI framework built on Dioxus) follows the same pattern as the Desktop example above — it runs natively so both <code>buildtime</code> and <code>ssr</code> features are available.',

    /* freya */
    'freya.title': 'Freya Integration',
    'freya.p': 'Freya is a native cross-platform GUI framework for Rust, so it can use <strong>both</strong> crate features. Use <code>buildtime</code> for embedded content in production builds, and optionally <code>ssr</code> in native targets when you want live filesystem content during development.',
    'freya.sub.buildtime': 'Buildtime (recommended)',
    'freya.sub.runtime': 'Runtime content with <code>ssr</code> (optional)',
    'freya.callout': 'Freya does <strong>not</strong> target <code>wasm32</code> today. That means <code>Collection::load()</code> is available for desktop/native apps, while <code>Collection::from_embedded()</code> remains ideal for self-contained binaries.',

    /* api — shared table headers */
    'api.th.method':  'Method',
    'api.th.feature': 'Feature',
    'api.th.desc':    'Description',
    'api.th.field':   'Field / Method',
    'api.th.type':    'Type',
    'api.th.param':   'Parameter',
    'api.th.crate':   'Crate',
    'api.th.purpose': 'Purpose',

    /* api — Collection<T> */
    'api.collection.p': 'The main type. <code>T</code> is your schema struct — it must implement <code>serde::Deserialize</code>.',
    'api.collection.load.desc': 'Reads all <code>.md</code> files from <code>dir</code> at runtime. Returns <code>Result&lt;Self, CollectionError&gt;</code>.',
    'api.collection.from-embedded.desc': 'Builds a collection from a <code>&amp;[EmbeddedEntry]</code> produced by <code>codegen::generate</code>. Returns <code>Result&lt;Self, CollectionError&gt;</code>.',
    'api.collection.entries.desc': 'Returns <code>&amp;[CollectionEntry&lt;T&gt;]</code> in load order.',
    'api.collection.into-entries.desc': 'Consumes the collection and returns <code>Vec&lt;CollectionEntry&lt;T&gt;&gt;</code>.',

    /* api — CollectionEntry<T> */
    'api.entry.p': 'A single entry in a collection.',
    'api.entry.slug.desc': 'Filename without extension, e.g. <code>"hello-world"</code>.',
    'api.entry.data.desc': 'The deserialized frontmatter matching your schema struct.',
    'api.entry.body.desc': 'Raw Markdown body (everything after the closing <code>---</code>).',
    'api.entry.render.desc': 'Renders <code>body</code> to an HTML string using <code>pulldown-cmark</code> with all extensions enabled.',

    /* api — EmbeddedEntry */
    'api.embedded.p': 'The element type of the array written by <code>codegen::generate</code>. You generally only interact with it indirectly via <code>Collection::from_embedded</code>, but the fields are public if you need to inspect them.',
    'api.embedded.slug.desc': 'Filename without extension.',
    'api.embedded.fm.desc': 'Raw YAML string between the <code>---</code> delimiters.',
    'api.embedded.body.desc': 'Raw Markdown body.',

    /* api — codegen */
    'api.codegen.p': 'Available only on non-WASM targets when the <code>buildtime</code> feature is active. Intended to be called from <code>build.rs</code>.',
    'api.codegen.dir.desc': 'Path to the directory containing <code>.md</code> files, relative to the crate root.',
    'api.codegen.name.desc': 'Name used for the generated file: <code>$OUT_DIR/{output_name}_collection.rs</code>.',
    'api.codegen.note': 'Also emits <code>cargo:rerun-if-changed</code> for the directory and each individual <code>.md</code> file so incremental builds work correctly.',

    /* dependencies */
    'deps.title': 'Dependencies',
    'deps.serde': 'Deserialising frontmatter YAML into your schema struct.',
    'deps.serde-yml': 'YAML parsing.',
    'deps.pulldown': 'Markdown → HTML rendering, with all extensions enabled.',
    'deps.thiserror': 'Error type derivation.',
    'deps.note': 'No Leptos dependency. No async runtime. No macros beyond standard derive.',
  },

  es: {
    /* page */
    'page.title': 'leptos-content-collection — Colecciones de contenido al estilo Astro para Rust',
    'page.desc':  'Un crate de Rust sin boilerplate para cargar colecciones de contenido Markdown tipadas. Funciona en tiempo de compilación o en runtime, agnóstico al framework.',

    /* nav */
    'menu.toggle':         'Menú',
    'nav.group.start':     'Primeros pasos',
    'nav.overview':        'Introducción',
    'nav.features':        'Características',
    'nav.content-format':  'Formato del contenido',
    'nav.markdown-styling':'Estilos de Markdown',
    'nav.group.usage':     'Uso',
    'nav.buildtime':       'Tiempo de compilación',
    'nav.default-badge':   'default',
    'nav.bt-install':      '1. Instalación',
    'nav.bt-buildrs':      '2. build.rs',
    'nav.bt-schema':       '3. Schema y carga',
    'nav.ssr':             'SSR / Runtime',
    'nav.leptos':          'Integración con Leptos',
    'nav.yew':             'Integración con Yew',
    'nav.dioxus':          'Integración con Dioxus',
    'nav.freya':           'Integración con Freya',
    'nav.group.ref':       'Referencia',
    'nav.group.meta':      'Meta',
    'nav.dependencies':    'Dependencias',

    /* hero */
    'hero.tagline': 'Colecciones de contenido al estilo Astro para Rust — define una struct de esquema, apúntala a un directorio de archivos Markdown y obtén una colección completamente tipada.',
    'badge.agnostic': 'agnóstico al framework',

    /* overview */
    'overview.title': 'Introducción',
    'overview.p1': '<code>leptos-content-collection</code> lleva el concepto de content collections, popularizado por <a href="https://docs.astro.build/en/guides/content-collections/" target="_blank">Astro</a>, al ecosistema de Rust. Cada directorio de archivos <code>.md</code> se convierte en una colección tipada: el frontmatter se deserializa en una struct de Rust en tiempo de compilación o de carga, y el cuerpo Markdown puede renderizarse a HTML bajo demanda.',
    'overview.callout': 'A pesar del nombre, este crate <strong>no tiene dependencia de Leptos</strong>. Funciona igual de bien con Axum, Actix-web, generadores de sitios estáticos, CLIs o cualquier otro proyecto Rust.',

    /* features */
    'features.title': 'Características',
    'feat.default': 'default',
    'feat.buildtime.desc': 'Los archivos de contenido se procesan durante <code>cargo build</code> y se embeben directamente en el binario. Sin acceso al sistema de archivos en runtime — funciona en servidores SSR y bundles WASM por igual.',
    'feat.ssr.desc': 'Lee archivos <code>.md</code> desde el sistema de archivos en tiempo de petición mediante <code>Collection::load()</code>. Útil cuando quieres actualizar el contenido sin recompilar.',
    'features.intro': 'Ambas features pueden estar activas al mismo tiempo. La tabla muestra qué habilita cada una:',
    'features.th.feature': 'Feature',
    'features.th.default': 'Por defecto',
    'features.th.methods': 'Método habilitado',

    /* content format */
    'format.title': 'Formato del contenido',
    'format.p': 'Cada archivo <code>.md</code> debe comenzar con un bloque de frontmatter YAML delimitado por <code>---</code>. El <strong>slug</strong> de cada entrada es el nombre del archivo sin la extensión <code>.md</code>.',
    'format.th.part': 'Parte',
    'format.th.desc': 'Descripción',
    'format.row.fm.label': 'Primer bloque <code>---</code>',
    'format.row.fm.desc': 'Frontmatter YAML. Se deserializa en tu struct de esquema.',
    'format.row.body.label': 'Cuerpo',
    'format.row.body.desc': 'Cuerpo Markdown en crudo. Accesible mediante <code>entry.body</code> y renderizable con <code>entry.render()</code>.',
    'format.row.slug.desc': 'Nombre de archivo sin <code>.md</code>, p. ej. <code>"hello-world"</code>.',
    'mdstyle.title': 'Estilos para Markdown renderizado',
    'mdstyle.p1': '<code>entry.render()</code> devuelve un string HTML. Renderízalo dentro de un contenedor (por ejemplo, <code>.md-content</code>) y limita tus estilos tipográficos a ese contenedor.',
    'mdstyle.note': 'Si el contenido puede provenir de fuentes no confiables, sanitiza el HTML generado antes de inyectarlo en el navegador.',
    'copy.btn': 'copiar',

    /* buildtime */
    'bt.title': 'Tiempo de compilación',
    'bt.p': 'El contenido se procesa durante <code>cargo build</code> y se embebe en el binario como literales de string estáticos. No se realizan llamadas a <code>std::fs</code> en runtime, por lo que el mismo binario funciona para servidores SSR y bundles WASM.',
    'bt.step1.title': 'Agrega la dependencia',
    'bt.step2.title': 'Crea un <code>build.rs</code>',
    'bt.step2.desc': '<code>codegen::generate(dir, name)</code> escanea <code>dir</code> en busca de archivos <code>.md</code>, embebe su contenido y escribe <code>$OUT_DIR/{name}_collection.rs</code>. También emite <code>cargo:rerun-if-changed</code> para que la compilación se repita cada vez que se agregue, edite o elimine un archivo de contenido.',
    'bt.step3.title': 'Define tu esquema y carga',

    /* ssr */
    'ssr.title': 'SSR / Runtime',
    'ssr.p': 'Activa la feature <code>ssr</code> para leer archivos desde el sistema de archivos en tiempo de petición. Útil cuando quieres editar el contenido sin recompilar.',
    'ssr.callout': '<code>Collection::load()</code> usa <code>std::fs</code> y no compilará para targets <code>wasm32</code>. En un proyecto Leptos, llámalo solo dentro de funciones <code>#[server]</code> o handlers de Axum.',

    /* leptos */
    'leptos.title': 'Integración con Leptos',
    'leptos.p': 'Una configuración típica de Leptos + Axum usa <strong>ambas features</strong>: <code>buildtime</code> para que el bundle WASM pueda acceder a los datos sin un round-trip al servidor, y <code>ssr</code> (opcionalmente) dentro de funciones <code>#[server]</code> para contenido recargable en desarrollo.',
    'leptos.sub3.title': 'app.rs — usando datos buildtime (funciona en SSR + WASM)',
    'leptos.callout': '<code>codegen::generate</code> está protegido con <code>cfg(not(target_arch = "wasm32"))</code> dentro del crate, por lo que nunca se compila en el bundle WASM — solo en <code>build.rs</code> (que corre en el host).',

    /* yew */
    'yew.title': 'Integración con Yew',
    'yew.p': 'Yew compila a WASM, así que solo funciona la feature <code>buildtime</code> — el contenido se embebe en compilación y no requiere acceso al sistema de archivos en runtime.',
    'yew.sub3.title': 'app.rs — componente de Yew',
    'yew.callout': 'La feature <code>ssr</code> usa <code>std::fs</code> y no compilará para targets <code>wasm32</code>. Mantente en <code>buildtime</code> para apps Yew puramente WASM.',

    /* dioxus */
    'dioxus.title': 'Integración con Dioxus',
    'dioxus.p': 'Dioxus soporta múltiples renderers (web, desktop, mobile). Para targets WASM usa <code>buildtime</code>; para desktop / nativo funcionan ambas features.',
    'dioxus.sub.wasm.title': 'Web (WASM) — buildtime',
    'dioxus.sub.desktop.title': 'Desktop — feature ssr (filesystem en runtime)',
    'dioxus.callout': 'Freya (un framework GUI nativo construido sobre Dioxus) sigue el mismo patrón que el ejemplo de Desktop: corre de forma nativa, así que están disponibles tanto <code>buildtime</code> como <code>ssr</code>.',

    /* freya */
    'freya.title': 'Integración con Freya',
    'freya.p': 'Freya es un framework GUI nativo y multiplataforma para Rust, así que puede usar <strong>ambas</strong> features del crate. Usa <code>buildtime</code> para contenido embebido en builds de producción y, opcionalmente, <code>ssr</code> en targets nativos cuando quieras contenido vivo desde el filesystem en desarrollo.',
    'freya.sub.buildtime': 'Buildtime (recomendado)',
    'freya.sub.runtime': 'Contenido en runtime con <code>ssr</code> (opcional)',
    'freya.callout': 'Freya <strong>no</strong> apunta a <code>wasm32</code> hoy. Eso significa que <code>Collection::load()</code> está disponible para apps desktop/nativas, mientras que <code>Collection::from_embedded()</code> sigue siendo ideal para binarios autocontenidos.',

    /* api — shared table headers */
    'api.th.method':  'Método',
    'api.th.feature': 'Feature',
    'api.th.desc':    'Descripción',
    'api.th.field':   'Campo / Método',
    'api.th.type':    'Tipo',
    'api.th.param':   'Parámetro',
    'api.th.crate':   'Crate',
    'api.th.purpose': 'Propósito',

    /* api — Collection<T> */
    'api.collection.p': 'El tipo principal. <code>T</code> es tu struct de esquema — debe implementar <code>serde::Deserialize</code>.',
    'api.collection.load.desc': 'Lee todos los archivos <code>.md</code> de <code>dir</code> en runtime. Devuelve <code>Result&lt;Self, CollectionError&gt;</code>.',
    'api.collection.from-embedded.desc': 'Construye una colección a partir de un <code>&amp;[EmbeddedEntry]</code> producido por <code>codegen::generate</code>. Devuelve <code>Result&lt;Self, CollectionError&gt;</code>.',
    'api.collection.entries.desc': 'Devuelve <code>&amp;[CollectionEntry&lt;T&gt;]</code> en orden de carga.',
    'api.collection.into-entries.desc': 'Consume la colección y devuelve <code>Vec&lt;CollectionEntry&lt;T&gt;&gt;</code>.',

    /* api — CollectionEntry<T> */
    'api.entry.p': 'Una entrada individual de la colección.',
    'api.entry.slug.desc': 'Nombre de archivo sin extensión, p. ej. <code>"hello-world"</code>.',
    'api.entry.data.desc': 'El frontmatter deserializado que coincide con tu struct de esquema.',
    'api.entry.body.desc': 'Cuerpo Markdown en crudo (todo lo que hay después del <code>---</code> de cierre).',
    'api.entry.render.desc': 'Renderiza <code>body</code> a un string HTML usando <code>pulldown-cmark</code> con todas las extensiones habilitadas.',

    /* api — EmbeddedEntry */
    'api.embedded.p': 'El tipo de elemento del array escrito por <code>codegen::generate</code>. Generalmente solo interactúas con él de forma indirecta mediante <code>Collection::from_embedded</code>, pero los campos son públicos si necesitas inspeccionarlos.',
    'api.embedded.slug.desc': 'Nombre de archivo sin extensión.',
    'api.embedded.fm.desc': 'String YAML en crudo entre los delimitadores <code>---</code>.',
    'api.embedded.body.desc': 'Cuerpo Markdown en crudo.',

    /* api — codegen */
    'api.codegen.p': 'Disponible solo en targets no-WASM cuando la feature <code>buildtime</code> está activa. Pensado para ser llamado desde <code>build.rs</code>.',
    'api.codegen.dir.desc': 'Ruta al directorio que contiene los archivos <code>.md</code>, relativa a la raíz del crate.',
    'api.codegen.name.desc': 'Nombre usado para el archivo generado: <code>$OUT_DIR/{output_name}_collection.rs</code>.',
    'api.codegen.note': 'También emite <code>cargo:rerun-if-changed</code> para el directorio y cada archivo <code>.md</code> individual, de modo que las compilaciones incrementales funcionen correctamente.',

    /* dependencies */
    'deps.title': 'Dependencias',
    'deps.serde': 'Deserializar el frontmatter YAML en tu struct de esquema.',
    'deps.serde-yml': 'Parseo de YAML.',
    'deps.pulldown': 'Renderizado Markdown → HTML, con todas las extensiones habilitadas.',
    'deps.thiserror': 'Derivación del tipo de error.',
    'deps.note': 'Sin dependencia de Leptos. Sin async runtime. Sin macros más allá del derive estándar.',
  },
};

/* ======================================================
   i18n engine
   ====================================================== */
function setLang(lang) {
  document.documentElement.lang = lang === 'es' ? 'es' : 'en';
  localStorage.setItem('lang', lang);

  document.title = t[lang]['page.title'];
  const metaDesc = document.querySelector('meta[name="description"]');
  if (metaDesc) metaDesc.setAttribute('content', t[lang]['page.desc']);

  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.dataset.i18n;
    if (t[lang][key] !== undefined) el.textContent = t[lang][key];
  });

  document.querySelectorAll('[data-i18n-html]').forEach(el => {
    const key = el.dataset.i18nHtml;
    if (t[lang][key] !== undefined) el.innerHTML = t[lang][key];
  });

  document.querySelectorAll('.lang-btn').forEach(btn => {
    btn.classList.toggle('active', btn.dataset.lang === lang);
  });
}

/* ---- Init lang from localStorage or browser preference ---- */
const savedLang = localStorage.getItem('lang');
const browserLang = navigator.language?.startsWith('es') ? 'es' : 'en';
setLang(savedLang || browserLang);

/* ---- Toggle buttons ---- */
document.querySelectorAll('.lang-btn').forEach(btn => {
  btn.addEventListener('click', () => setLang(btn.dataset.lang));
});

/* ======================================================
   Copy buttons
   ====================================================== */
document.querySelectorAll('.copy-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const pre = btn.closest('.code-block').querySelector('pre');
    navigator.clipboard.writeText(pre.innerText).then(() => {
      const original = btn.textContent;
      btn.textContent = '✓';
      btn.classList.add('copied');
      setTimeout(() => {
        btn.textContent = original;
        btn.classList.remove('copied');
      }, 2000);
    });
  });
});

/* ======================================================
   Active nav link on scroll
   ====================================================== */
const sections = document.querySelectorAll('[data-section]');
const navLinks  = document.querySelectorAll('.sidebar-link[href^="#"]');

const observer = new IntersectionObserver(entries => {
  entries.forEach(entry => {
    if (entry.isIntersecting) {
      const id = entry.target.getAttribute('data-section');
      navLinks.forEach(link => {
        link.classList.toggle('active', link.getAttribute('href') === `#${id}`);
      });
    }
  });
}, { rootMargin: '-20% 0px -70% 0px' });

sections.forEach(s => observer.observe(s));

/* ======================================================
   Mobile sidebar
   ====================================================== */
const sidebar = document.querySelector('.sidebar');
const toggle  = document.querySelector('.menu-toggle');
const overlay = document.querySelector('.overlay');

toggle?.addEventListener('click', () => {
  sidebar.classList.toggle('open');
  overlay.classList.toggle('visible');
});

overlay?.addEventListener('click', () => {
  sidebar.classList.remove('open');
  overlay.classList.remove('visible');
});

navLinks.forEach(link => {
  link.addEventListener('click', () => {
    sidebar.classList.remove('open');
    overlay.classList.remove('visible');
  });
});
