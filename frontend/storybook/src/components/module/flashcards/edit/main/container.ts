import {argsToAttrs} from "@utils/attributes";
import {mapToString, arrayCount} from "@utils/array";
import "@elements/core/module-page/grid-resize";
import "@elements/module/flashcards/edit/main/container";
import "~/components/module/_groups/cards/edit/main/card-pair/card";
import {Mode} from "@elements/module/flashcards/edit/sidebar/option";
import {Card, Args as CardArgs} from "~/components/module/_groups/cards/play/card";
export default {
    title: "Module / Flashcards / Edit / Main" 
}

interface Args {
    mode: Mode,
}

const DEFAULT_ARGS:Args = {
    mode: "pair",
}

export const Container = (props?:Partial<Args> & {content?: string}) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    const {mode} = props;
    return `

      <module-page-grid-resize>
	<flashcards-main slot="main">
		${mode === "single" ? renderSingle() : renderPair()}
	</flashcards-main>
      </module-page-grid-resize>
      `;
}

function renderCard(flipped: boolean) {
	return Card({
		contentMode: "image",
		theme: "happy-brush",
		size: "flashcards",
		flipped,
		flipOnHover: true
	});
}
function renderSingle() {
	return `
		${renderCard(true)}
	`
}
function renderPair() {
	return `
		${renderCard(true)}
		${renderCard(false)}
	`
}
Container.args= DEFAULT_ARGS;

Container.argTypes = {
  mode: {
    control: {
      type: 'inline-radio',
      options: ["single", "pair"]
    }
  },
}