// create a custom element with the custom_tag
// `custom_tag` - a user defined tag with a dash separator
//          example: my-button, advance-element, ui-editor
//  `adapter` - The class name of the object which defines the behavior the custom element
//      example _Button__CustomElement
//  `superClass` - The adapter class inherits to.
//      example: HTMLElement
export function register_custom_element(custom_tag, adapter, superClass){
    // https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements
    window.customElements.define(custom_tag,
        class extends window[superClass]{
            constructor(){
                super();
                console.log("outer html: {}", this.outerHTML);
                this.instance = new window[adapter](this);
            }

            static get observedAttributes(){
                return window[adapter].observedAttributes;
            }

            connectedCallback(){
                this.instance.connectedCallback();
            }
            disconnectedCallback(){
                this.instance.disconnectedCallback();
            }
            adoptedCallback(){
                this.instance.adoptedCallback();
            }
            attributeChangedCallback(){
                this.instance.attributeChangedCallback();
            }

            appendChild(child){
                console.log("appending a child:", child);
                this.instance.appendChild(child);
            }

        }
    );
}
