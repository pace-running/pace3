import * as Yup from 'yup';

export const ChangeThemeSchema = Yup.object().shape({
  eventTitle: Yup.string()
    .min(3, 'Name des Events soll mindestens drei Zeichen enthalten!')
    .max(50, 'Der Name des Events darf maximal 50 Zeichen beinhalten!'),
  eventDescription: Yup.string()
    .min(3, 'Beschreibung soll mindestens drei Zeichen enthalten!')
    .max(2000, 'Der Beschreibung des Events darf maximal 2000 Zeichen beinhalten!'),
  closedRegistrationMessage: Yup.string().max(500, 'Die Nachricht darf maximal 500 Zeichen enthalten!'),
  isRegistrationOpen: Yup.boolean(),
  tshirtsEnabled: Yup.boolean(),
  decentralSignupEnabled: Yup.boolean(),
});

export type ChangeThemeFormValues = {
  eventTitle: string;
  eventDescription: string;
  closedRegistrationMessage: string;
  isRegistrationOpen: boolean;
  tshirtsEnabled: boolean;
  decentralSignupEnabled: boolean;
};
