import { ChangeEventHandler, EventHandler, FormEventHandler, MouseEventHandler } from "react";

type ButtonProps = {
    name: string;
    label: string;
    type: "submit" | "button";
    onSubmit?: EventHandler<any>;
    onClick?: MouseEventHandler;
    disabled?: boolean;
    styling?: string;
}

const Button: React.FC<ButtonProps> = (props) => {
    return (
        <button type={props.type} name={props.name} onSubmit={props.onSubmit} onClick={props.onClick} disabled={props.disabled??false} className={props.styling}>
            {props.label}
        </button>
    );
}

export default Button;