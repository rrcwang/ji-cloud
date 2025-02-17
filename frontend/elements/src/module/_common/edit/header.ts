import { LitElement, html, css, customElement, property } from 'lit-element';
import {classMap} from "lit-html/directives/class-map";
import {nothing} from "lit-html";
import "@elements/core/tooltips/info";
import {ModuleKind, STR_MODULE_DISPLAY_NAME, STR_MODULE_HEADER_TOOLTIP_BODY} from "@elements/module/_common/types";


const STR_TOOLTIP_GETTING_STARTED = "Getting started";


@customElement('module-header')
export class _ extends LitElement {
  static get styles() {
      return [css`
          .topRight {
              position: absolute;
              top: 21px;
              right: 40px;
              display: flex;
              gap: 24px;
              align-items: center;
          }
          .title {
              margin-top: 90px;
              font-size: 32px;
              font-weight: 900;
              letter-spacing: -0.32px;
              text-align: left;
              color: var(--dark-blue-4);
          }
    `];
  }

  @property()
  moduleKind:ModuleKind = "memory";

  imgRef:HTMLElement | undefined;

  //instead of firstUpdated since tooltip needs the size of the image to position correctly
  onImageLoaded() {
      this.imgRef = this.shadowRoot?.getElementById("gear-img") as HTMLElement;
      this.requestUpdate();
  }

  render() {
      const {imgRef, moduleKind} = this;

      const title = STR_MODULE_DISPLAY_NAME[moduleKind];

      return html`
          <section>
                  <div class="topRight">
                      <slot name="controller"></slot>
                      <img-ui @image-load=${this.onImageLoaded} id="gear-img" path="module/_common/edit/header/jiggling-gear.png"></img-ui>
                      ${imgRef ? renderTooltip(moduleKind, imgRef) : nothing} 
                  </div>
                  <div class="title">${title}</div>
                  <slot></slot>
          </section>
      `
  }
}

function renderTooltip(moduleKind:ModuleKind, targetRef:HTMLElement) {
    const body = STR_MODULE_HEADER_TOOLTIP_BODY[moduleKind];
    if(!body) {
        return nothing;
    }

    const showId = `header-intro-${moduleKind}`;
    return html`<tooltip-info placement="bottom-end" .target=${targetRef} .arrowOffset=${35} title="${STR_TOOLTIP_GETTING_STARTED}" body="${body}" showId="${showId}" closeable></tooltip-info>`
}
