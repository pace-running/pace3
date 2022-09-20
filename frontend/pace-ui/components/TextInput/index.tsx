type InputProps = {
  type: "email" | "text" | "number";
  placeholder?: string;
  name: string;
  label: string;
  helperLabel?: string;
};
const TextInput: React.FC<InputProps> = (props) => {
  return (
    <div className="mb-3">
      {props.helperLabel && <p style={{marginBottom: "1px"}}>{props.helperLabel}</p>}
      <label htmlFor={props.name} className="form-label">
        {props.label}
      </label>
      <input
        type={props.type}
        className="form-control"
        id={props.name}
        placeholder={props.placeholder}
      />
    </div>
  );
};

export default TextInput;