// create a custom element with the custom_tag
// `custom_tag` - a user defined tag with a dash separator
//          example: my-button, advance-element, ui-editor
//  `adapter` - The class name of the object which defines the behavior the custom element
//      example Button__CustomElement
export function register_custom_element(custom_tag, adapter){
    // https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements
    if (window.customElements.get(custom_tag) === undefined ){
        window.customElements.define(custom_tag,
            class extends HTMLElement{
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
                attributeChangedCallback(name, oldValue, newValue){
                    this.instance.attributeChangedCallback(name, oldValue, newValue);
                }

                appendChild(child){
                    console.log("appending a child:", child);
                    this.instance.appendChild(child);
                }

            }
        );
    }else{
        console.log("tag [" + custom_tag + "] is already defined");
    }
}
