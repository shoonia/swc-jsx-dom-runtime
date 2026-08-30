<div />;
<button />;
<p><span /></p>;
<p><span /><strong /></p>;
<div><p><span>hello</span></p></div>;
<div tabIndex={0} />;
<a className="my-class" />;
<a xlink:href="https://example.com" />;
<input required readOnly disabled spellcheck />;
<div aria-hidden data-test />;
<div aria-hidden={false} data-test={true} />;
<div on:click={() => console.log("clicked")} />;
<div on:click={() => 1} on:mouseOver={() => 2} />;
<input type="text" on:focusIn={{ handleEvent }} />;
<div prop:foo={{}}/>;
<div prop:foo={[]}/>;
<div prop:hello="world" />;
<div prop:hello={`hello ${user}`} />;
<div attr:test="value" />;
<div attr:foo="value" attr:bar={1} />;
<div attr:foo={'hello'} prop:bar-baz={{}} />;
<div attr:foo={data} prop:bar-baz={data} />;
<div attr:foo={`hello ${user }`} />;
<button type="submit" onclick={handleClick} />;
<div style="color: red; background-color: blue;" />;
<div style={{ color: 'red', backgroundColor: 'blue' }} />;
<span attributes={[]} />;
<iframe dataset={{ foo: 'bar' }} />;
<div ref={e => e.focus()} />;
<div ref={e => e.focus()} attr:test="value" prop:foo="bar" style={style} attributes={attrs} onclick={handleClick} />;
<div children={<span />} />;
<code children=<span /> />;
<div children={<><small /><strong /></>} />;
<div children={<Component />} />;
<em children={10} />;
<li children="1" children="2" />;
<ol children="skip">used</ol>;
<img alt="
        text
" />;
<img alt='
        text
' />;
