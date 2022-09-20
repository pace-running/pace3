

type InputProps = {
    type: "email" | "text" | "number";
    placeholder?: string;
    name: string;
    label: string;
}
const TextInput: React.FC<InputProps> = (props) => {
  return (
    <div className="mb-3">
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
