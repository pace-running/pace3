import React, { EventHandler, SyntheticEvent } from 'react';

type ModalProps = {
  name: string;
  children: React.ReactNode;
  onClose: EventHandler<SyntheticEvent>;
  open: boolean;
};

const Modal: React.FC<ModalProps> = props => {
  if (!props.open) {
    return null;
  }
  return (
    <div>
      <div className='overlay' onClick={props.onClose}></div>
      <div className='modal-window' id={props.name} style={{ textAlign: 'center' }}>
        <button type='button' className='close-modal' aria-label='Close' onClick={props.onClose}>
          <span aria-hidden='true'>
            <svg
              xmlns='http://www.w3.org/2000/svg'
              width='24'
              height='24'
              fill='currentColor'
              className='bi bi-x-circle'
              viewBox='0 0 16 16'
            >
              <path d='M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z' />
              <path d='M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z' />
            </svg>
          </span>
        </button>
        {props.children}
      </div>
    </div>
  );
};

export default Modal;
