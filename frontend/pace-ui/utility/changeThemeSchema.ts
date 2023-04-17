import * as Yup from 'yup';

export const ChangeThemeSchema = Yup.object().shape({
  eventTitle: Yup.string().min(3, 'Name des Events soll mindestens drei Zeichen enthalten!'),
  eventDescription: Yup.string().min(3, 'Beschreibung soll mindestens drei Zeichen enthalten!'),
  closedRegistrationMessage: Yup.string().min(3, 'Nachricht soll mindestens drei Zeichen enthalten!'),
  isRegistrationOpen: Yup.boolean(),
  tshirtsEnabled: Yup.boolean()
});

export type ChangeThemeFormValues = {
  eventTitle: string;
  eventDescription: string;
  closedRegistrationMessage: string;
  isRegistrationOpen: boolean;
  tshirtsEnabled: boolean;
};
