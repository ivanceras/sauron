export function register_custom_element(custom_tag, adapter, superClass){
    window.customElements.define(custom_tag,
                class extends window[superClass]{
                    constructor(){
                        super();
                        this.instance = new window.components[adapter](this);
                    }

                    static get observedAttributes(){
                        return window.components[adapter].observed_attributes();
                    }

                    connectedCallback(){
                        this.instance.connected_callback();
                    }
                    disconnectedCallback(){
                        this.instance.disconnected_callback();
                    }
                    adoptedCallback(){
                        this.instance.adopted_callback();
                    }
                    attributeChangedCallback(){
                        this.instance.attribute_changed_callback();
                    }

                }
            );
    }
