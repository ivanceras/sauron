/**
* create a custom element with the custom_tag
* @param {string} custom_tag - a user defined tag with a dash separator
*          example: my-button, advance-element, ui-editor
* @param {string} adapterClassName - The class name of the object which defines the behavior the custom element
*      example Button__CustomElement
*/
export function register_custom_element(custom_tag, adapterClassName){
    define_custom_element(custom_tag, adapterClassName);
}


function define_custom_element(custom_tag, adapterClassName){
    // https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements
    if (window.customElements.get(custom_tag) === undefined ){
        let adapter = window[adapterClassName];
        window.customElements.define(custom_tag,
            class extends HTMLElement{
                constructor(){
                    super();
                    this.instance = new adapter(this);
                }

                static get observedAttributes(){
                    return adapter.observedAttributes;
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
    }
}
