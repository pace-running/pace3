import { number } from "yup";

interface InfoRequestData {
  // RunnerInfoRequestData
  firstname: string;
  lastname: string;
  team: string;
  email: string;
  repeat: string;
  starting_point: string;
  running_level: string;
  donation: string;
  confirm: string;

  // ShippingInfoRequestData
  tshirt_toggle: string;
  tshirt_model: string;
  tshirt_size: string;
  country: string;
  address_firstname: string;
  address_lastname: string;
  street_name: string;
  house_number: string;
  address_extra: string;
  postal_code: string;
  city: string;
}

interface InfoResponseData {
  runner_id: string;
  start_number: string;
  donation: string;
  tshirt_cost: string;
  payment: string;
  email_provided: boolean;
  verification_code: string;
}

interface StatusResponseData {
  runner_id: string;
  start_number: string;
  donation: string;
  tshirt_cost: string;
  payment: string;
  is_paid: boolean;

  is_tshirt_booked: boolean;
  tshirt_model: string;
  tshirt_size: string;
  country: string;
  address_firstname: string;
  address_lastname: string;
  street_name: string;
  house_number: string;
  address_extra: string;
  postal_code: string;
  city: string;
  delivery_status: string;
}

interface RunnerResponseData {
  id: number; // necessary???
  firstname: string;
  lastname: string;
  team: string;
  email: string;
  repeat: string;
  start_number: string;
  starting_point: string;
  running_level: string;
  donation: string;
  confirm: string;
  verification_code: string;
  reason_for_payment: string;
  payment_status: bool;
}

interface FullRunnerData {
  runner_id: string;
  firstname: string;
  lastname: string;
  team: string;
  email: string;
  starting_point: string;
  running_level: string;
  donation: string;

  is_tshirt_booked: boolean;
  tshirt_model: string;
  tshirt_size: string;
  country: string;
  address_firstname: string;
  address_lastname: string;
  street_name: string;
  house_number: string;
  address_extra: string;
  postal_code: string;
  city: string;

  start_number: string;
  verification_code: string;
  reason_for_payment: string;
  payment_status: bool;
  delivery_status: string;
  payment_confirmation_mail_sent: boolean;
}

interface RejectedTransaction {
  id: number;
  runner_ids: string;
  reasons_for_payment: string;
  payment_amount: string;
  expected_amount: string;
  currency: string;
  date_of_payment: string;
  payer_name: string;
  iban: string;
}
