import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/user/email/pages/email-send";
import "@elements/core/buttons/rectangle";
import "@elements/entry/user/email/buttons/email-send";
import { Mode as ConfirmationMode }  from "@elements/entry/user/email/buttons/email-send";

export default {
  title: 'Entry / User / Email / Pages',
}
const STR_CHANGE = "Change email account";

interface Args {
}

const DEFAULT_ARGS:Args = {
}

export const EmailSend = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <page-email-send>
            <button-email-send slot="send"></button-email-send>
        </page-email-send>
    `
}

EmailSend.args = DEFAULT_ARGS;