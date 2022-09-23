interface RunnerInfoRequestData {
    firstname: string,
    lastname: string,
    team: string,
    email: string,
    repeat: string,
    starting_point: string,
    running_level: string,
    donation: string,
    confirm: string,
}

interface ShippingInfoRequestData {
    tshirt_toggle: string,
    tshirt_model: string,
    tshirt_size: string,
    country: string,
    address_firstname: string,
    address_lastname: string,
    street_name: string,
    house_number: string,
    address_extra: string,
    postal_code: string,
    city: string,
}

interface InfoRequestData {
    runner_info_data: RunnerInfoRequestData,
    shipping_info_dat: ShippingInfoRequestData,
}