import { LitElement, html, css, customElement, property} from 'lit-element';
import {nothing} from "lit-html";
import {colorStyles} from "@elements/_styles/colors";
import {arrayCount} from "@utils/array";

@customElement('steps-nav')
export class _ extends LitElement {

  static get styles() {
    return [colorStyles, css`
      .steps {
        display: flex;
        align-items: center;
        justify-content: space-between;
      }
      .separator {
        background-color: rgba(161,168,173,1);
        height: 1px;
        width: 100%;
      }
    `];
  }

  @property({type: Number})
  count:number = 0;

  // Define the element's template
  render() {
    const {count} = this;

    return html`
        <div class="steps">
          ${arrayCount(count)
              .map(step => makeStep(step, count))
          }
        </div>
    `;
  }
}

const makeStep = (step:number, lastStep:number) => html`
  <slot name="slot-${step}"></slot>
  ${step !== lastStep 
      ? html`<div class="separator"></div>` 
      : nothing
   }
`
