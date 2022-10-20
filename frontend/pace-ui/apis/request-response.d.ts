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
  payment: string;
  email_provided: boolean;
  verification_code: string;
}

interface StatusResponseData {
  runner_id: string;
  start_number: string;
  donation: string;
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
