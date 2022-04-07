# druid-rune
Experiments combining druid and rune.

# GOAL
* CSS

```css
/** div { display:flex; } */
div {
    padding: 10px;
}

button {
    background-color: #6f6a6a;
}

#btn_red {
    background-color: red;
}
```

* Layout
```html
<body> <!-- window container. Show as default -->
    <div id="container">
        Button Test
        <button id="mybtn">OK</button>
        <button id="btn_red">Red Button</button>
    </div>
</body>

<div class="btn_cn"> <!-- custom component. invisible -->
    
</div>
```

* Script
```rune
fn on_load() {
    Q("#mybtn").click( || {
        print("clicked");
    });
    
    Q("#btn_red").click( || {
        Q.ajax("http://github.com", |resp| {
            print( resp.text() );
        });
    } );
}
```