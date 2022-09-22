import { EventHandler, FormEventHandler, MouseEventHandler } from "react";

type ButtonProps = {
    name: string;
    label: string;
    type: "submit" | "button";
    onSubmit?: FormEventHandler;
    onClick?: MouseEventHandler;
    disabled?: boolean;
}

const Button: React.FC<ButtonProps> = (props) => {
    return (
        <button type={props.type} name={props.name} onSubmit={props.onSubmit} onClick={props.onClick} disabled={props.disabled??false}>
            {props.label}
        </button>
    );
}

export default Button;