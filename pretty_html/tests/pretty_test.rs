use pretty_html::PretyHtmlBuilder;
use txml::build_template;

#[test]
fn basic_nested_elements<'a>() {
    let lil_template: &str = "<article><header>boop</header><p>hai :3</p></article>";
    let expected: &str = "<article>
	<header>
		boop
	</header>
	<p>
		hai :3
	</p>
</article>
";

    let mut builder = PretyHtmlBuilder::new();
    build_template(&mut builder, lil_template);

    let results = builder.build();

    assert_eq!(expected, results);
}

#[test]
fn basic_nested_elements_with_attributes<'a>() {
    let lil_template: &str = "<article class=\"article\" data-blip=boop><header aria-label=\"article title\">boop</header><p>hai :3</p></article>";
    let expected: &str = "<article class=\"article\" data-blip=boop>
	<header aria-label=\"article title\">
		boop
	</header>
	<p>
		hai :3
	</p>
</article>
";

    let mut builder = PretyHtmlBuilder::new();
    build_template(&mut builder, lil_template);

    let results = builder.build();

    assert_eq!(expected, results);
}