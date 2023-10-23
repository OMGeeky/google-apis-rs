use super::*;
/// Physical adaptations made to the property in consideration of varying levels of human physical ability.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Accessibility {
    /// Mobility accessible. Throughout the property there are physical adaptations to ease the stay of a person in a wheelchair, such as auto-opening doors, wide elevators, wide bathrooms or ramps.
    #[serde(rename="mobilityAccessible")]
    
    pub mobility_accessible: Option<bool>,
    /// Mobility accessible elevator. A lift that transports people from one level to another and is built to accommodate a wheelchair-using passenger owing to the width of its doors and placement of call buttons.
    #[serde(rename="mobilityAccessibleElevator")]
    
    pub mobility_accessible_elevator: Option<bool>,
    /// Mobility accessible elevator exception.
    #[serde(rename="mobilityAccessibleElevatorException")]
    
    pub mobility_accessible_elevator_exception: Option<AccessibilityMobilityAccessibleElevatorExceptionEnum>,
    /// Mobility accessible exception.
    #[serde(rename="mobilityAccessibleException")]
    
    pub mobility_accessible_exception: Option<AccessibilityMobilityAccessibleExceptionEnum>,
    /// Mobility accessible parking. The presence of a marked, designated area of prescribed size in which only registered, labeled vehicles transporting a person with physical challenges may park.
    #[serde(rename="mobilityAccessibleParking")]
    
    pub mobility_accessible_parking: Option<bool>,
    /// Mobility accessible parking exception.
    #[serde(rename="mobilityAccessibleParkingException")]
    
    pub mobility_accessible_parking_exception: Option<AccessibilityMobilityAccessibleParkingExceptionEnum>,
    /// Mobility accessible pool. A swimming pool equipped with a mechanical chair that can be lowered and raised for the purpose of moving physically challenged guests into and out of the pool. May be powered by electricity or water. Also known as pool lift.
    #[serde(rename="mobilityAccessiblePool")]
    
    pub mobility_accessible_pool: Option<bool>,
    /// Mobility accessible pool exception.
    #[serde(rename="mobilityAccessiblePoolException")]
    
    pub mobility_accessible_pool_exception: Option<AccessibilityMobilityAccessiblePoolExceptionEnum>,
}

impl client::Part for Accessibility {}


/// Amenities and features related to leisure and play.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activities {
    /// Beach access. The hotel property is in close proximity to a beach and offers a way to get to that beach. This can include a route to the beach such as stairs down if hotel is on a bluff, or a short trail. Not the same as beachfront (with beach access, the hotel's proximity is close to but not right on the beach).
    #[serde(rename="beachAccess")]
    
    pub beach_access: Option<bool>,
    /// Beach access exception.
    #[serde(rename="beachAccessException")]
    
    pub beach_access_exception: Option<ActivityBeachAccessExceptionEnum>,
    /// Breach front. The hotel property is physically located on the beach alongside an ocean, sea, gulf, or bay. It is not on a lake, river, stream, or pond. The hotel is not separated from the beach by a public road allowing vehicular, pedestrian, or bicycle traffic.
    #[serde(rename="beachFront")]
    
    pub beach_front: Option<bool>,
    /// Beach front exception.
    #[serde(rename="beachFrontException")]
    
    pub beach_front_exception: Option<ActivityBeachFrontExceptionEnum>,
    /// Bicycle rental. The hotel owns bicycles that it permits guests to borrow and use. Can be free or for a fee.
    #[serde(rename="bicycleRental")]
    
    pub bicycle_rental: Option<bool>,
    /// Bicycle rental exception.
    #[serde(rename="bicycleRentalException")]
    
    pub bicycle_rental_exception: Option<ActivityBicycleRentalExceptionEnum>,
    /// Boutique stores. There are stores selling clothing, jewelry, art and decor either on hotel premises or very close by. Does not refer to the hotel gift shop or convenience store.
    #[serde(rename="boutiqueStores")]
    
    pub boutique_stores: Option<bool>,
    /// Boutique stores exception.
    #[serde(rename="boutiqueStoresException")]
    
    pub boutique_stores_exception: Option<ActivityBoutiqueStoresExceptionEnum>,
    /// Casino. A space designated for gambling and gaming featuring croupier-run table and card games, as well as electronic slot machines. May be on hotel premises or located nearby.
    
    pub casino: Option<bool>,
    /// Casino exception.
    #[serde(rename="casinoException")]
    
    pub casino_exception: Option<ActivityCasinoExceptionEnum>,
    /// Free bicycle rental. The hotel owns bicycles that it permits guests to borrow and use for free.
    #[serde(rename="freeBicycleRental")]
    
    pub free_bicycle_rental: Option<bool>,
    /// Free bicycle rental exception.
    #[serde(rename="freeBicycleRentalException")]
    
    pub free_bicycle_rental_exception: Option<ActivityFreeBicycleRentalExceptionEnum>,
    /// Free watercraft rental. The hotel owns watercraft that it permits guests to borrow and use for free.
    #[serde(rename="freeWatercraftRental")]
    
    pub free_watercraft_rental: Option<bool>,
    /// Free Watercraft rental exception.
    #[serde(rename="freeWatercraftRentalException")]
    
    pub free_watercraft_rental_exception: Option<ActivityFreeWatercraftRentalExceptionEnum>,
    /// Game room. There is a room at the hotel containing electronic machines for play such as pinball, prize machines, driving simulators, and other items commonly found at a family fun center or arcade. May also include non-electronic games like pool, foosball, darts, and more. May or may not be designed for children. Also known as arcade, fun room, or family fun center.
    #[serde(rename="gameRoom")]
    
    pub game_room: Option<bool>,
    /// Game room exception.
    #[serde(rename="gameRoomException")]
    
    pub game_room_exception: Option<ActivityGameRoomExceptionEnum>,
    /// Golf. There is a golf course on hotel grounds or there is a nearby, independently run golf course that allows use by hotel guests. Can be free or for a fee.
    
    pub golf: Option<bool>,
    /// Golf exception.
    #[serde(rename="golfException")]
    
    pub golf_exception: Option<ActivityGolfExceptionEnum>,
    /// Horseback riding. The hotel has a horse barn onsite or an affiliation with a nearby barn to allow for guests to sit astride a horse and direct it to walk, trot, cantor, gallop and/or jump. Can be in a riding ring, on designated paths, or in the wilderness. May or may not involve instruction.
    #[serde(rename="horsebackRiding")]
    
    pub horseback_riding: Option<bool>,
    /// Horseback riding exception.
    #[serde(rename="horsebackRidingException")]
    
    pub horseback_riding_exception: Option<ActivityHorsebackRidingExceptionEnum>,
    /// Nightclub. There is a room at the hotel with a bar, a dance floor, and seating where designated staffers play dance music. There may also be a designated area for the performance of live music, singing and comedy acts.
    
    pub nightclub: Option<bool>,
    /// Nightclub exception.
    #[serde(rename="nightclubException")]
    
    pub nightclub_exception: Option<ActivityNightclubExceptionEnum>,
    /// Private beach. The beach which is in close proximity to the hotel is open only to guests.
    #[serde(rename="privateBeach")]
    
    pub private_beach: Option<bool>,
    /// Private beach exception.
    #[serde(rename="privateBeachException")]
    
    pub private_beach_exception: Option<ActivityPrivateBeachExceptionEnum>,
    /// Scuba. The provision for guests to dive under naturally occurring water fitted with a self-contained underwater breathing apparatus (SCUBA) for the purpose of exploring underwater life. Apparatus consists of a tank providing oxygen to the diver through a mask. Requires certification of the diver and supervision. The hotel may have the activity at its own waterfront or have an affiliation with a nearby facility. Required equipment is most often supplied to guests. Can be free or for a fee. Not snorkeling. Not done in a swimming pool.
    
    pub scuba: Option<bool>,
    /// Scuba exception.
    #[serde(rename="scubaException")]
    
    pub scuba_exception: Option<ActivityScubaExceptionEnum>,
    /// Snorkeling. The provision for guests to participate in a recreational water activity in which swimmers wear a diving mask, a simple, shaped breathing tube and flippers/swim fins for the purpose of exploring below the surface of an ocean, gulf or lake. Does not usually require user certification or professional supervision. Equipment may or may not be available for rent or purchase. Not scuba diving.
    
    pub snorkeling: Option<bool>,
    /// Snorkeling exception.
    #[serde(rename="snorkelingException")]
    
    pub snorkeling_exception: Option<ActivitySnorkelingExceptionEnum>,
    /// Tennis. The hotel has the requisite court(s) on site or has an affiliation with a nearby facility for the purpose of providing guests with the opportunity to play a two-sided court-based game in which players use a stringed racquet to hit a ball across a net to the side of the opposing player. The court can be indoors or outdoors. Instructors, racquets and balls may or may not be provided.
    
    pub tennis: Option<bool>,
    /// Tennis exception.
    #[serde(rename="tennisException")]
    
    pub tennis_exception: Option<ActivityTennisExceptionEnum>,
    /// Water skiing. The provision of giving guests the opportunity to be pulled across naturally occurring water while standing on skis and holding a tow rope attached to a motorboat. Can occur on hotel premises or at a nearby waterfront. Most often performed in a lake or ocean.
    #[serde(rename="waterSkiing")]
    
    pub water_skiing: Option<bool>,
    /// Water skiing exception.
    #[serde(rename="waterSkiingException")]
    
    pub water_skiing_exception: Option<ActivityWaterSkiingExceptionEnum>,
    /// Watercraft rental. The hotel owns water vessels that it permits guests to borrow and use. Can be free or for a fee. Watercraft may include boats, pedal boats, rowboats, sailboats, powerboats, canoes, kayaks, or personal watercraft (such as a Jet Ski).
    #[serde(rename="watercraftRental")]
    
    pub watercraft_rental: Option<bool>,
    /// Watercraft rental exception.
    #[serde(rename="watercraftRentalException")]
    
    pub watercraft_rental_exception: Option<ActivityWatercraftRentalExceptionEnum>,
}

impl client::Part for Activities {}


/// Features of the property of specific interest to the business traveler.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Business {
    /// Business center. A designated room at the hotel with one or more desks and equipped with guest-use computers, printers, fax machines and/or photocopiers. May or may not be open 24/7. May or may not require a key to access. Not a meeting room or conference room.
    #[serde(rename="businessCenter")]
    
    pub business_center: Option<bool>,
    /// Business center exception.
    #[serde(rename="businessCenterException")]
    
    pub business_center_exception: Option<BusinesBusinessCenterExceptionEnum>,
    /// Meeting rooms. Rooms at the hotel designated for business-related gatherings. Rooms are usually equipped with tables or desks, office chairs and audio/visual facilities to allow for presentations and conference calls. Also known as conference rooms.
    #[serde(rename="meetingRooms")]
    
    pub meeting_rooms: Option<bool>,
    /// Meeting rooms count. The number of meeting rooms at the property.
    #[serde(rename="meetingRoomsCount")]
    
    pub meeting_rooms_count: Option<i32>,
    /// Meeting rooms count exception.
    #[serde(rename="meetingRoomsCountException")]
    
    pub meeting_rooms_count_exception: Option<BusinesMeetingRoomsCountExceptionEnum>,
    /// Meeting rooms exception.
    #[serde(rename="meetingRoomsException")]
    
    pub meeting_rooms_exception: Option<BusinesMeetingRoomsExceptionEnum>,
}

impl client::Part for Business {}


/// The ways in which the property provides guests with the ability to access the internet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Connectivity {
    /// Free wifi. The hotel offers guests wifi for free.
    #[serde(rename="freeWifi")]
    
    pub free_wifi: Option<bool>,
    /// Free wifi exception.
    #[serde(rename="freeWifiException")]
    
    pub free_wifi_exception: Option<ConnectivityFreeWifiExceptionEnum>,
    /// Public area wifi available. Guests have the ability to wirelessly connect to the internet in the areas of the hotel accessible to anyone. Can be free or for a fee.
    #[serde(rename="publicAreaWifiAvailable")]
    
    pub public_area_wifi_available: Option<bool>,
    /// Public area wifi available exception.
    #[serde(rename="publicAreaWifiAvailableException")]
    
    pub public_area_wifi_available_exception: Option<ConnectivityPublicAreaWifiAvailableExceptionEnum>,
    /// Public internet terminal. An area of the hotel supplied with computers and designated for the purpose of providing guests with the ability to access the internet.
    #[serde(rename="publicInternetTerminal")]
    
    pub public_internet_terminal: Option<bool>,
    /// Public internet terminal exception.
    #[serde(rename="publicInternetTerminalException")]
    
    pub public_internet_terminal_exception: Option<ConnectivityPublicInternetTerminalExceptionEnum>,
    /// Wifi available. The hotel provides the ability for guests to wirelessly connect to the internet. Can be in the public areas of the hotel and/or in the guest rooms. Can be free or for a fee.
    #[serde(rename="wifiAvailable")]
    
    pub wifi_available: Option<bool>,
    /// Wifi available exception.
    #[serde(rename="wifiAvailableException")]
    
    pub wifi_available_exception: Option<ConnectivityWifiAvailableExceptionEnum>,
}

impl client::Part for Connectivity {}


/// An eco certificate awarded to the hotel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EcoCertification {
    /// Whether the eco certificate was awarded or not.
    
    pub awarded: Option<bool>,
    /// Awarded exception.
    #[serde(rename="awardedException")]
    
    pub awarded_exception: Option<EcoCertificationAwardedExceptionEnum>,
    /// Required. The eco certificate.
    #[serde(rename="ecoCertificate")]
    
    pub eco_certificate: Option<EcoCertificationEcoCertificateEnum>,
}

impl client::Part for EcoCertification {}


/// Energy efficiency practices implemented at the hotel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnergyEfficiency {
    /// Carbon free energy sources. Property sources carbon-free electricity via at least one of the following methods: on-site clean energy generation, power purchase agreement(s) with clean energy generators, green power provided by electricity supplier, or purchases of Energy Attribute Certificates (such as Renewable Energy Certificates or Guarantees of Origin).
    #[serde(rename="carbonFreeEnergySources")]
    
    pub carbon_free_energy_sources: Option<bool>,
    /// Carbon free energy sources exception.
    #[serde(rename="carbonFreeEnergySourcesException")]
    
    pub carbon_free_energy_sources_exception: Option<EnergyEfficiencyCarbonFreeEnergySourcesExceptionEnum>,
    /// Energy conservation program. The property tracks corporate-level Scope 1 and 2 GHG emissions, and Scope 3 emissions if available. The property has a commitment to implement initiatives that reduce GHG emissions year over year. The property has shown an absolute reduction in emissions for at least 2 years. Emissions are either verfied by a third-party and/or published in external communications.
    #[serde(rename="energyConservationProgram")]
    
    pub energy_conservation_program: Option<bool>,
    /// Energy conservation program exception.
    #[serde(rename="energyConservationProgramException")]
    
    pub energy_conservation_program_exception: Option<EnergyEfficiencyEnergyConservationProgramExceptionEnum>,
    /// Energy efficient heating and cooling systems. The property doesn't use chlorofluorocarbon (CFC)-based refrigerants in heating, ventilating, and air-conditioning systems unless a third-party audit shows it's not economically feasible. The CFC-based refrigerants which are used should have a Global Warming Potential (GWP) ≤ 10. The property uses occupancy sensors on HVAC systems in back-of-house spaces, meeting rooms, and other low-traffic areas.
    #[serde(rename="energyEfficientHeatingAndCoolingSystems")]
    
    pub energy_efficient_heating_and_cooling_systems: Option<bool>,
    /// Energy efficient heating and cooling systems exception.
    #[serde(rename="energyEfficientHeatingAndCoolingSystemsException")]
    
    pub energy_efficient_heating_and_cooling_systems_exception: Option<EnergyEfficiencyEnergyEfficientHeatingAndCoolingSystemsExceptionEnum>,
    /// Energy efficient lighting. At least 75% of the property's lighting is energy efficient, using lighting that is more than 45 lumens per watt – typically LED or CFL lightbulbs.
    #[serde(rename="energyEfficientLighting")]
    
    pub energy_efficient_lighting: Option<bool>,
    /// Energy efficient lighting exception.
    #[serde(rename="energyEfficientLightingException")]
    
    pub energy_efficient_lighting_exception: Option<EnergyEfficiencyEnergyEfficientLightingExceptionEnum>,
    /// Energy saving thermostats. The property installed energy-saving thermostats throughout the building to conserve energy when rooms or areas are not in use. Energy-saving thermostats are devices that control heating/cooling in the building by learning temperature preferences and automatically adjusting to energy-saving temperatures as the default. The thermostats are automatically set to a temperature between 68-78 degrees F (20-26 °C), depending on seasonality. In the winter, set the thermostat to 68°F (20°C) when the room is occupied, lowering room temperature when unoccupied. In the summer, set the thermostat to 78°F (26°C) when the room is occupied.
    #[serde(rename="energySavingThermostats")]
    
    pub energy_saving_thermostats: Option<bool>,
    /// Energy saving thermostats exception.
    #[serde(rename="energySavingThermostatsException")]
    
    pub energy_saving_thermostats_exception: Option<EnergyEfficiencyEnergySavingThermostatsExceptionEnum>,
    /// Output only. Green building design. True if BREEAM-* or LEED-* certified.
    #[serde(rename="greenBuildingDesign")]
    
    pub green_building_design: Option<bool>,
    /// Output only. Green building design exception.
    #[serde(rename="greenBuildingDesignException")]
    
    pub green_building_design_exception: Option<EnergyEfficiencyGreenBuildingDesignExceptionEnum>,
    /// Independent organization audits energy use. The property conducts an energy audit at least every 5 years, the results of which are either verified by a third-party and/or published in external communications. An energy audit is a detailed assessment of the facility which provides recommendations to existing operations and procedures to improve energy efficiency, available incentives or rebates,and opportunities for improvements through renovations or upgrades. Examples of organizations that conduct credible third party audits include: Engie Impact, DNV GL (EU), Dexma, and local utility providers (they often provide energy and water audits).
    #[serde(rename="independentOrganizationAuditsEnergyUse")]
    
    pub independent_organization_audits_energy_use: Option<bool>,
    /// Independent organization audits energy use exception.
    #[serde(rename="independentOrganizationAuditsEnergyUseException")]
    
    pub independent_organization_audits_energy_use_exception: Option<EnergyEfficiencyIndependentOrganizationAuditsEnergyUseExceptionEnum>,
}

impl client::Part for EnergyEfficiency {}


/// Enhanced cleaning measures implemented by the hotel during COVID-19.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnhancedCleaning {
    /// Commercial-grade disinfectant used to clean the property.
    #[serde(rename="commercialGradeDisinfectantCleaning")]
    
    pub commercial_grade_disinfectant_cleaning: Option<bool>,
    /// Commercial grade disinfectant cleaning exception.
    #[serde(rename="commercialGradeDisinfectantCleaningException")]
    
    pub commercial_grade_disinfectant_cleaning_exception: Option<EnhancedCleaningCommercialGradeDisinfectantCleaningExceptionEnum>,
    /// Enhanced cleaning of common areas.
    #[serde(rename="commonAreasEnhancedCleaning")]
    
    pub common_areas_enhanced_cleaning: Option<bool>,
    /// Common areas enhanced cleaning exception.
    #[serde(rename="commonAreasEnhancedCleaningException")]
    
    pub common_areas_enhanced_cleaning_exception: Option<EnhancedCleaningCommonAreasEnhancedCleaningExceptionEnum>,
    /// Employees trained in COVID-19 cleaning procedures.
    #[serde(rename="employeesTrainedCleaningProcedures")]
    
    pub employees_trained_cleaning_procedures: Option<bool>,
    /// Employees trained cleaning procedures exception.
    #[serde(rename="employeesTrainedCleaningProceduresException")]
    
    pub employees_trained_cleaning_procedures_exception: Option<EnhancedCleaningEmployeesTrainedCleaningProceduresExceptionEnum>,
    /// Employees trained in thorough hand-washing.
    #[serde(rename="employeesTrainedThoroughHandWashing")]
    
    pub employees_trained_thorough_hand_washing: Option<bool>,
    /// Employees trained thorough hand washing exception.
    #[serde(rename="employeesTrainedThoroughHandWashingException")]
    
    pub employees_trained_thorough_hand_washing_exception: Option<EnhancedCleaningEmployeesTrainedThoroughHandWashingExceptionEnum>,
    /// Employees wear masks, face shields, and/or gloves.
    #[serde(rename="employeesWearProtectiveEquipment")]
    
    pub employees_wear_protective_equipment: Option<bool>,
    /// Employees wear protective equipment exception.
    #[serde(rename="employeesWearProtectiveEquipmentException")]
    
    pub employees_wear_protective_equipment_exception: Option<EnhancedCleaningEmployeesWearProtectiveEquipmentExceptionEnum>,
    /// Enhanced cleaning of guest rooms.
    #[serde(rename="guestRoomsEnhancedCleaning")]
    
    pub guest_rooms_enhanced_cleaning: Option<bool>,
    /// Guest rooms enhanced cleaning exception.
    #[serde(rename="guestRoomsEnhancedCleaningException")]
    
    pub guest_rooms_enhanced_cleaning_exception: Option<EnhancedCleaningGuestRoomsEnhancedCleaningExceptionEnum>,
}

impl client::Part for EnhancedCleaning {}


/// Services and amenities for families and young guests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Families {
    /// Babysitting. Child care that is offered by hotel staffers or coordinated by hotel staffers with local child care professionals. Can be free or for a fee.
    
    pub babysitting: Option<bool>,
    /// Babysitting exception.
    #[serde(rename="babysittingException")]
    
    pub babysitting_exception: Option<FamilyBabysittingExceptionEnum>,
    /// Kids activities. Recreational options such as sports, films, crafts and games designed for the enjoyment of children and offered at the hotel. May or may not be supervised. May or may not be at a designated time or place. Cab be free or for a fee.
    #[serde(rename="kidsActivities")]
    
    pub kids_activities: Option<bool>,
    /// Kids activities exception.
    #[serde(rename="kidsActivitiesException")]
    
    pub kids_activities_exception: Option<FamilyKidsActivitiesExceptionEnum>,
    /// Kids club. An organized program of group activities held at the hotel and designed for the enjoyment of children. Facilitated by hotel staff (or staff procured by the hotel) in an area(s) designated for the purpose of entertaining children without their parents. May include games, outings, water sports, team sports, arts and crafts, and films. Usually has set hours. Can be free or for a fee. Also known as Kids Camp or Kids program.
    #[serde(rename="kidsClub")]
    
    pub kids_club: Option<bool>,
    /// Kids club exception.
    #[serde(rename="kidsClubException")]
    
    pub kids_club_exception: Option<FamilyKidsClubExceptionEnum>,
    /// Kids friendly. The hotel has one or more special features for families with children, such as reduced rates, child-sized beds, kids' club, babysitting service, or suitable place to play on premises.
    #[serde(rename="kidsFriendly")]
    
    pub kids_friendly: Option<bool>,
    /// Kids friendly exception.
    #[serde(rename="kidsFriendlyException")]
    
    pub kids_friendly_exception: Option<FamilyKidsFriendlyExceptionEnum>,
}

impl client::Part for Families {}


/// Meals, snacks, and beverages available at the property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FoodAndDrink {
    /// Bar. A designated room, lounge or area of an on-site restaurant with seating at a counter behind which a hotel staffer takes the guest's order and provides the requested alcoholic drink. Can be indoors or outdoors. Also known as Pub.
    
    pub bar: Option<bool>,
    /// Bar exception.
    #[serde(rename="barException")]
    
    pub bar_exception: Option<FoodAndDrinkBarExceptionEnum>,
    /// Breakfast available. The morning meal is offered to all guests. Can be free or for a fee.
    #[serde(rename="breakfastAvailable")]
    
    pub breakfast_available: Option<bool>,
    /// Breakfast available exception.
    #[serde(rename="breakfastAvailableException")]
    
    pub breakfast_available_exception: Option<FoodAndDrinkBreakfastAvailableExceptionEnum>,
    /// Breakfast buffet. Breakfast meal service where guests serve themselves from a variety of dishes/foods that are put out on a table.
    #[serde(rename="breakfastBuffet")]
    
    pub breakfast_buffet: Option<bool>,
    /// Breakfast buffet exception.
    #[serde(rename="breakfastBuffetException")]
    
    pub breakfast_buffet_exception: Option<FoodAndDrinkBreakfastBuffetExceptionEnum>,
    /// Buffet. A type of meal where guests serve themselves from a variety of dishes/foods that are put out on a table. Includes lunch and/or dinner meals. A breakfast-only buffet is not sufficient.
    
    pub buffet: Option<bool>,
    /// Buffet exception.
    #[serde(rename="buffetException")]
    
    pub buffet_exception: Option<FoodAndDrinkBuffetExceptionEnum>,
    /// Dinner buffet. Dinner meal service where guests serve themselves from a variety of dishes/foods that are put out on a table.
    #[serde(rename="dinnerBuffet")]
    
    pub dinner_buffet: Option<bool>,
    /// Dinner buffet exception.
    #[serde(rename="dinnerBuffetException")]
    
    pub dinner_buffet_exception: Option<FoodAndDrinkDinnerBuffetExceptionEnum>,
    /// Free breakfast. Breakfast is offered for free to all guests. Does not apply if limited to certain room packages.
    #[serde(rename="freeBreakfast")]
    
    pub free_breakfast: Option<bool>,
    /// Free breakfast exception.
    #[serde(rename="freeBreakfastException")]
    
    pub free_breakfast_exception: Option<FoodAndDrinkFreeBreakfastExceptionEnum>,
    /// Restaurant. A business onsite at the hotel that is open to the public as well as guests, and offers meals and beverages to consume at tables or counters. May or may not include table service. Also known as cafe, buffet, eatery. A "breakfast room" where the hotel serves breakfast only to guests (not the general public) does not count as a restaurant.
    
    pub restaurant: Option<bool>,
    /// Restaurant exception.
    #[serde(rename="restaurantException")]
    
    pub restaurant_exception: Option<FoodAndDrinkRestaurantExceptionEnum>,
    /// Restaurants count. The number of restaurants at the hotel.
    #[serde(rename="restaurantsCount")]
    
    pub restaurants_count: Option<i32>,
    /// Restaurants count exception.
    #[serde(rename="restaurantsCountException")]
    
    pub restaurants_count_exception: Option<FoodAndDrinkRestaurantsCountExceptionEnum>,
    /// Room service. A hotel staffer delivers meals prepared onsite to a guest's room as per their request. May or may not be available during specific hours. Services should be available to all guests (not based on rate/room booked/reward program, etc).
    #[serde(rename="roomService")]
    
    pub room_service: Option<bool>,
    /// Room service exception.
    #[serde(rename="roomServiceException")]
    
    pub room_service_exception: Option<FoodAndDrinkRoomServiceExceptionEnum>,
    /// Table service. A restaurant in which a staff member is assigned to a guest's table to take their order, deliver and clear away food, and deliver the bill, if applicable. Also known as sit-down restaurant.
    #[serde(rename="tableService")]
    
    pub table_service: Option<bool>,
    /// Table service exception.
    #[serde(rename="tableServiceException")]
    
    pub table_service_exception: Option<FoodAndDrinkTableServiceExceptionEnum>,
    /// 24hr room service. Room service is available 24 hours a day.
    #[serde(rename="twentyFourHourRoomService")]
    
    pub twenty_four_hour_room_service: Option<bool>,
    /// 24hr room service exception.
    #[serde(rename="twentyFourHourRoomServiceException")]
    
    pub twenty_four_hour_room_service_exception: Option<FoodAndDrinkTwentyFourHourRoomServiceExceptionEnum>,
    /// Vending machine. A glass-fronted mechanized cabinet displaying and dispensing snacks and beverages for purchase by coins, paper money and/or credit cards.
    #[serde(rename="vendingMachine")]
    
    pub vending_machine: Option<bool>,
    /// Vending machine exception.
    #[serde(rename="vendingMachineException")]
    
    pub vending_machine_exception: Option<FoodAndDrinkVendingMachineExceptionEnum>,
}

impl client::Part for FoodAndDrink {}


/// Response message for LodgingService.GetGoogleUpdatedLodging
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lodging get google updated locations](LocationLodgingGetGoogleUpdatedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetGoogleUpdatedLodgingResponse {
    /// Required. The fields in the Lodging that have been updated by Google. Repeated field items are not individually specified.
    #[serde(rename="diffMask")]
    
    pub diff_mask: Option<client::FieldMask>,
    /// Required. The Google updated Lodging.
    
    pub lodging: Option<Lodging>,
}

impl client::ResponseResult for GetGoogleUpdatedLodgingResponse {}


/// Features and available amenities in the guest unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuestUnitFeatures {
    /// Bungalow or villa. An independent structure that is part of a hotel or resort that is rented to one party for a vacation stay. The hotel or resort may be completely comprised of bungalows or villas, or they may be one of several guestroom options. Guests in the bungalows or villas most often have the same, if not more, amenities and services offered to guests in other guestroom types.
    #[serde(rename="bungalowOrVilla")]
    
    pub bungalow_or_villa: Option<bool>,
    /// Bungalow or villa exception.
    #[serde(rename="bungalowOrVillaException")]
    
    pub bungalow_or_villa_exception: Option<GuestUnitFeatureBungalowOrVillaExceptionEnum>,
    /// Connecting unit available. A guestroom type that features access to an adjacent guestroom for the purpose of booking both rooms. Most often used by families who need more than one room to accommodate the number of people in their group.
    #[serde(rename="connectingUnitAvailable")]
    
    pub connecting_unit_available: Option<bool>,
    /// Connecting unit available exception.
    #[serde(rename="connectingUnitAvailableException")]
    
    pub connecting_unit_available_exception: Option<GuestUnitFeatureConnectingUnitAvailableExceptionEnum>,
    /// Executive floor. A floor of the hotel where the guestrooms are only bookable by members of the hotel's frequent guest membership program. Benefits of this room class include access to a designated lounge which may or may not feature free breakfast, cocktails or other perks specific to members of the program.
    #[serde(rename="executiveFloor")]
    
    pub executive_floor: Option<bool>,
    /// Executive floor exception.
    #[serde(rename="executiveFloorException")]
    
    pub executive_floor_exception: Option<GuestUnitFeatureExecutiveFloorExceptionEnum>,
    /// Max adult occupants count. The total number of adult guests allowed to stay overnight in the guestroom.
    #[serde(rename="maxAdultOccupantsCount")]
    
    pub max_adult_occupants_count: Option<i32>,
    /// Max adult occupants count exception.
    #[serde(rename="maxAdultOccupantsCountException")]
    
    pub max_adult_occupants_count_exception: Option<GuestUnitFeatureMaxAdultOccupantsCountExceptionEnum>,
    /// Max child occupants count. The total number of children allowed to stay overnight in the room.
    #[serde(rename="maxChildOccupantsCount")]
    
    pub max_child_occupants_count: Option<i32>,
    /// Max child occupants count exception.
    #[serde(rename="maxChildOccupantsCountException")]
    
    pub max_child_occupants_count_exception: Option<GuestUnitFeatureMaxChildOccupantsCountExceptionEnum>,
    /// Max occupants count. The total number of guests allowed to stay overnight in the guestroom.
    #[serde(rename="maxOccupantsCount")]
    
    pub max_occupants_count: Option<i32>,
    /// Max occupants count exception.
    #[serde(rename="maxOccupantsCountException")]
    
    pub max_occupants_count_exception: Option<GuestUnitFeatureMaxOccupantsCountExceptionEnum>,
    /// Private home. A privately owned home (house, townhouse, apartment, cabin, bungalow etc) that may or not serve as the owner's residence, but is rented out in its entirety or by the room(s) to paying guest(s) for vacation stays. Not for lease-based, long-term residency.
    #[serde(rename="privateHome")]
    
    pub private_home: Option<bool>,
    /// Private home exception.
    #[serde(rename="privateHomeException")]
    
    pub private_home_exception: Option<GuestUnitFeaturePrivateHomeExceptionEnum>,
    /// Suite. A guestroom category that implies both a bedroom area and a separate living area. There may or may not be full walls and doors separating the two areas, but regardless, they are very distinct. Does not mean a couch or chair in a bedroom.
    
    pub suite: Option<bool>,
    /// Suite exception.
    #[serde(rename="suiteException")]
    
    pub suite_exception: Option<GuestUnitFeatureSuiteExceptionEnum>,
    /// Tier. Classification of the unit based on available features/amenities. A non-standard tier is only permitted if at least one other unit type falls under the standard tier.
    
    pub tier: Option<GuestUnitFeatureTierEnum>,
    /// Tier exception.
    #[serde(rename="tierException")]
    
    pub tier_exception: Option<GuestUnitFeatureTierExceptionEnum>,
    /// Features available in the living areas in the guest unit.
    #[serde(rename="totalLivingAreas")]
    
    pub total_living_areas: Option<LivingArea>,
    /// Views available from the guest unit itself.
    
    pub views: Option<ViewsFromUnit>,
}

impl client::Part for GuestUnitFeatures {}


/// A specific type of unit primarily defined by its features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GuestUnitType {
    /// Required. Unit or room code identifiers for a single GuestUnitType. Each code must be unique within a Lodging instance.
    
    pub codes: Option<Vec<String>>,
    /// Features and available amenities of the GuestUnitType.
    
    pub features: Option<GuestUnitFeatures>,
    /// Required. Short, English label or name of the GuestUnitType. Target <50 chars.
    
    pub label: Option<String>,
}

impl client::Part for GuestUnitType {}


/// Health and safety measures implemented by the hotel during COVID-19.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HealthAndSafety {
    /// Enhanced cleaning measures implemented by the hotel during COVID-19.
    #[serde(rename="enhancedCleaning")]
    
    pub enhanced_cleaning: Option<EnhancedCleaning>,
    /// Increased food safety measures implemented by the hotel during COVID-19.
    #[serde(rename="increasedFoodSafety")]
    
    pub increased_food_safety: Option<IncreasedFoodSafety>,
    /// Minimized contact measures implemented by the hotel during COVID-19.
    #[serde(rename="minimizedContact")]
    
    pub minimized_contact: Option<MinimizedContact>,
    /// Personal protection measures implemented by the hotel during COVID-19.
    #[serde(rename="personalProtection")]
    
    pub personal_protection: Option<PersonalProtection>,
    /// Physical distancing measures implemented by the hotel during COVID-19.
    #[serde(rename="physicalDistancing")]
    
    pub physical_distancing: Option<PhysicalDistancing>,
}

impl client::Part for HealthAndSafety {}


/// Conveniences provided in guest units to facilitate an easier, more comfortable stay.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Housekeeping {
    /// Daily housekeeping. Guest units are cleaned by hotel staff daily during guest's stay.
    #[serde(rename="dailyHousekeeping")]
    
    pub daily_housekeeping: Option<bool>,
    /// Daily housekeeping exception.
    #[serde(rename="dailyHousekeepingException")]
    
    pub daily_housekeeping_exception: Option<HousekeepingDailyHousekeepingExceptionEnum>,
    /// Housekeeping available. Guest units are cleaned by hotel staff during guest's stay. Schedule may vary from daily, weekly, or specific days of the week.
    #[serde(rename="housekeepingAvailable")]
    
    pub housekeeping_available: Option<bool>,
    /// Housekeeping available exception.
    #[serde(rename="housekeepingAvailableException")]
    
    pub housekeeping_available_exception: Option<HousekeepingHousekeepingAvailableExceptionEnum>,
    /// Turndown service. Hotel staff enters guest units to prepare the bed for sleep use. May or may not include some light housekeeping. May or may not include an evening snack or candy. Also known as evening service.
    #[serde(rename="turndownService")]
    
    pub turndown_service: Option<bool>,
    /// Turndown service exception.
    #[serde(rename="turndownServiceException")]
    
    pub turndown_service_exception: Option<HousekeepingTurndownServiceExceptionEnum>,
}

impl client::Part for Housekeeping {}


/// Increased food safety measures implemented by the hotel during COVID-19.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IncreasedFoodSafety {
    /// Additional sanitation in dining areas.
    #[serde(rename="diningAreasAdditionalSanitation")]
    
    pub dining_areas_additional_sanitation: Option<bool>,
    /// Dining areas additional sanitation exception.
    #[serde(rename="diningAreasAdditionalSanitationException")]
    
    pub dining_areas_additional_sanitation_exception: Option<IncreasedFoodSafetyDiningAreasAdditionalSanitationExceptionEnum>,
    /// Disposable flatware.
    #[serde(rename="disposableFlatware")]
    
    pub disposable_flatware: Option<bool>,
    /// Disposable flatware exception.
    #[serde(rename="disposableFlatwareException")]
    
    pub disposable_flatware_exception: Option<IncreasedFoodSafetyDisposableFlatwareExceptionEnum>,
    /// Additional safety measures during food prep and serving.
    #[serde(rename="foodPreparationAndServingAdditionalSafety")]
    
    pub food_preparation_and_serving_additional_safety: Option<bool>,
    /// Food preparation and serving additional safety exception.
    #[serde(rename="foodPreparationAndServingAdditionalSafetyException")]
    
    pub food_preparation_and_serving_additional_safety_exception: Option<IncreasedFoodSafetyFoodPreparationAndServingAdditionalSafetyExceptionEnum>,
    /// Individually-packaged meals.
    #[serde(rename="individualPackagedMeals")]
    
    pub individual_packaged_meals: Option<bool>,
    /// Individual packaged meals exception.
    #[serde(rename="individualPackagedMealsException")]
    
    pub individual_packaged_meals_exception: Option<IncreasedFoodSafetyIndividualPackagedMealsExceptionEnum>,
    /// Single-use menus.
    #[serde(rename="singleUseFoodMenus")]
    
    pub single_use_food_menus: Option<bool>,
    /// Single use food menus exception.
    #[serde(rename="singleUseFoodMenusException")]
    
    pub single_use_food_menus_exception: Option<IncreasedFoodSafetySingleUseFoodMenusExceptionEnum>,
}

impl client::Part for IncreasedFoodSafety {}


/// Language spoken by at least one staff member.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageSpoken {
    /// Required. The BCP-47 language code for the spoken language. Currently accepted codes: ar, de, en, es, fil, fr, hi, id, it, ja, ko, nl, pt, ru, vi, yue, zh.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// At least one member of the staff can speak the language.
    
    pub spoken: Option<bool>,
    /// Spoken exception.
    #[serde(rename="spokenException")]
    
    pub spoken_exception: Option<LanguageSpokenSpokenExceptionEnum>,
}

impl client::Part for LanguageSpoken {}


/// An individual room, such as kitchen, bathroom, bedroom, within a bookable guest unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LivingArea {
    /// Accessibility features of the living area.
    
    pub accessibility: Option<LivingAreaAccessibility>,
    /// Information about eating features in the living area.
    
    pub eating: Option<LivingAreaEating>,
    /// Features in the living area.
    
    pub features: Option<LivingAreaFeatures>,
    /// Information about the layout of the living area.
    
    pub layout: Option<LivingAreaLayout>,
    /// Information about sleeping features in the living area.
    
    pub sleeping: Option<LivingAreaSleeping>,
}

impl client::Part for LivingArea {}


/// Accessibility features of the living area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LivingAreaAccessibility {
    /// ADA compliant unit. A guestroom designed to accommodate the physical challenges of a guest with mobility and/or auditory and/or visual issues, as determined by legislative policy. Usually features enlarged doorways, roll-in showers with seats, bathroom grab bars, and communication equipment for the hearing and sight challenged.
    #[serde(rename="adaCompliantUnit")]
    
    pub ada_compliant_unit: Option<bool>,
    /// ADA compliant unit exception.
    #[serde(rename="adaCompliantUnitException")]
    
    pub ada_compliant_unit_exception: Option<LivingAreaAccessibilityAdaCompliantUnitExceptionEnum>,
    /// Hearing-accessible doorbell. A visual indicator(s) of a knock or ring at the door.
    #[serde(rename="hearingAccessibleDoorbell")]
    
    pub hearing_accessible_doorbell: Option<bool>,
    /// Hearing-accessible doorbell exception.
    #[serde(rename="hearingAccessibleDoorbellException")]
    
    pub hearing_accessible_doorbell_exception: Option<LivingAreaAccessibilityHearingAccessibleDoorbellExceptionEnum>,
    /// Hearing-accessible fire alarm. A device that gives warning of a fire through flashing lights.
    #[serde(rename="hearingAccessibleFireAlarm")]
    
    pub hearing_accessible_fire_alarm: Option<bool>,
    /// Hearing-accessible fire alarm exception.
    #[serde(rename="hearingAccessibleFireAlarmException")]
    
    pub hearing_accessible_fire_alarm_exception: Option<LivingAreaAccessibilityHearingAccessibleFireAlarmExceptionEnum>,
    /// Hearing-accessible unit. A guestroom designed to accommodate the physical challenges of a guest with auditory issues.
    #[serde(rename="hearingAccessibleUnit")]
    
    pub hearing_accessible_unit: Option<bool>,
    /// Hearing-accessible unit exception.
    #[serde(rename="hearingAccessibleUnitException")]
    
    pub hearing_accessible_unit_exception: Option<LivingAreaAccessibilityHearingAccessibleUnitExceptionEnum>,
    /// Mobility-accessible bathtub. A bathtub that accomodates the physically challenged with additional railings or hand grips, a transfer seat or lift, and/or a door to enable walking into the tub.
    #[serde(rename="mobilityAccessibleBathtub")]
    
    pub mobility_accessible_bathtub: Option<bool>,
    /// Mobility-accessible bathtub exception.
    #[serde(rename="mobilityAccessibleBathtubException")]
    
    pub mobility_accessible_bathtub_exception: Option<LivingAreaAccessibilityMobilityAccessibleBathtubExceptionEnum>,
    /// Mobility-accessible shower. A shower with an enlarged door or access point to accommodate a wheelchair or a waterproof seat for the physically challenged.
    #[serde(rename="mobilityAccessibleShower")]
    
    pub mobility_accessible_shower: Option<bool>,
    /// Mobility-accessible shower exception.
    #[serde(rename="mobilityAccessibleShowerException")]
    
    pub mobility_accessible_shower_exception: Option<LivingAreaAccessibilityMobilityAccessibleShowerExceptionEnum>,
    /// Mobility-accessible toilet. A toilet with a higher seat, grab bars, and/or a larger area around it to accommodate the physically challenged.
    #[serde(rename="mobilityAccessibleToilet")]
    
    pub mobility_accessible_toilet: Option<bool>,
    /// Mobility-accessible toilet exception.
    #[serde(rename="mobilityAccessibleToiletException")]
    
    pub mobility_accessible_toilet_exception: Option<LivingAreaAccessibilityMobilityAccessibleToiletExceptionEnum>,
    /// Mobility-accessible unit. A guestroom designed to accommodate the physical challenges of a guest with mobility and/or auditory and/or visual issues. Usually features enlarged doorways, roll-in showers with seats, bathroom grab bars, and communication equipment for the hearing and sight challenged.
    #[serde(rename="mobilityAccessibleUnit")]
    
    pub mobility_accessible_unit: Option<bool>,
    /// Mobility-accessible unit exception.
    #[serde(rename="mobilityAccessibleUnitException")]
    
    pub mobility_accessible_unit_exception: Option<LivingAreaAccessibilityMobilityAccessibleUnitExceptionEnum>,
}

impl client::Part for LivingAreaAccessibility {}


/// Information about eating features in the living area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LivingAreaEating {
    /// Coffee maker. An electric appliance that brews coffee by heating and forcing water through ground coffee.
    #[serde(rename="coffeeMaker")]
    
    pub coffee_maker: Option<bool>,
    /// Coffee maker exception.
    #[serde(rename="coffeeMakerException")]
    
    pub coffee_maker_exception: Option<LivingAreaEatingCoffeeMakerExceptionEnum>,
    /// Cookware. Kitchen pots, pans and utensils used in connection with the preparation of food.
    
    pub cookware: Option<bool>,
    /// Cookware exception.
    #[serde(rename="cookwareException")]
    
    pub cookware_exception: Option<LivingAreaEatingCookwareExceptionEnum>,
    /// Dishwasher. A counter-height electrical cabinet containing racks for dirty dishware, cookware and cutlery, and a dispenser for soap built into the pull-down door. The cabinet is attached to the plumbing system to facilitate the automatic cleaning of its contents.
    
    pub dishwasher: Option<bool>,
    /// Dishwasher exception.
    #[serde(rename="dishwasherException")]
    
    pub dishwasher_exception: Option<LivingAreaEatingDishwasherExceptionEnum>,
    /// Indoor grill. Metal grates built into an indoor cooktop on which food is cooked over an open flame or electric heat source.
    #[serde(rename="indoorGrill")]
    
    pub indoor_grill: Option<bool>,
    /// Indoor grill exception.
    #[serde(rename="indoorGrillException")]
    
    pub indoor_grill_exception: Option<LivingAreaEatingIndoorGrillExceptionEnum>,
    /// Kettle. A covered container with a handle and a spout used for boiling water.
    
    pub kettle: Option<bool>,
    /// Kettle exception.
    #[serde(rename="kettleException")]
    
    pub kettle_exception: Option<LivingAreaEatingKettleExceptionEnum>,
    /// Kitchen available. An area of the guestroom designated for the preparation and storage of food via the presence of a refrigerator, cook top, oven and sink, as well as cutlery, dishes and cookware. Usually includes small appliances such a coffee maker and a microwave. May or may not include an automatic dishwasher.
    #[serde(rename="kitchenAvailable")]
    
    pub kitchen_available: Option<bool>,
    /// Kitchen available exception.
    #[serde(rename="kitchenAvailableException")]
    
    pub kitchen_available_exception: Option<LivingAreaEatingKitchenAvailableExceptionEnum>,
    /// Microwave. An electric oven that quickly cooks and heats food by microwave energy. Smaller than a standing or wall mounted oven. Usually placed on a kitchen counter, a shelf or tabletop or mounted above a cooktop.
    
    pub microwave: Option<bool>,
    /// Microwave exception.
    #[serde(rename="microwaveException")]
    
    pub microwave_exception: Option<LivingAreaEatingMicrowaveExceptionEnum>,
    /// Minibar. A small refrigerated cabinet in the guestroom containing bottles/cans of soft drinks, mini bottles of alcohol, and snacks. The items are most commonly available for a fee.
    
    pub minibar: Option<bool>,
    /// Minibar exception.
    #[serde(rename="minibarException")]
    
    pub minibar_exception: Option<LivingAreaEatingMinibarExceptionEnum>,
    /// Outdoor grill. Metal grates on which food is cooked over an open flame or electric heat source. Part of an outdoor apparatus that supports the grates. Also known as barbecue grill or barbecue.
    #[serde(rename="outdoorGrill")]
    
    pub outdoor_grill: Option<bool>,
    /// Outdoor grill exception.
    #[serde(rename="outdoorGrillException")]
    
    pub outdoor_grill_exception: Option<LivingAreaEatingOutdoorGrillExceptionEnum>,
    /// Oven. A temperature controlled, heated metal cabinet powered by gas or electricity in which food is placed for the purpose of cooking or reheating.
    
    pub oven: Option<bool>,
    /// Oven exception.
    #[serde(rename="ovenException")]
    
    pub oven_exception: Option<LivingAreaEatingOvenExceptionEnum>,
    /// Refrigerator. A large, climate-controlled electrical cabinet with vertical doors. Built for the purpose of chilling and storing perishable foods.
    
    pub refrigerator: Option<bool>,
    /// Refrigerator exception.
    #[serde(rename="refrigeratorException")]
    
    pub refrigerator_exception: Option<LivingAreaEatingRefrigeratorExceptionEnum>,
    /// Sink. A basin with a faucet attached to a water source and used for the purpose of washing and rinsing.
    
    pub sink: Option<bool>,
    /// Sink exception.
    #[serde(rename="sinkException")]
    
    pub sink_exception: Option<LivingAreaEatingSinkExceptionEnum>,
    /// Snackbar. A small cabinet in the guestroom containing snacks. The items are most commonly available for a fee.
    
    pub snackbar: Option<bool>,
    /// Snackbar exception.
    #[serde(rename="snackbarException")]
    
    pub snackbar_exception: Option<LivingAreaEatingSnackbarExceptionEnum>,
    /// Stove. A kitchen appliance powered by gas or electricity for the purpose of creating a flame or hot surface on which pots of food can be cooked. Also known as cooktop or hob.
    
    pub stove: Option<bool>,
    /// Stove exception.
    #[serde(rename="stoveException")]
    
    pub stove_exception: Option<LivingAreaEatingStoveExceptionEnum>,
    /// Tea station. A small area with the supplies needed to heat water and make tea.
    #[serde(rename="teaStation")]
    
    pub tea_station: Option<bool>,
    /// Tea station exception.
    #[serde(rename="teaStationException")]
    
    pub tea_station_exception: Option<LivingAreaEatingTeaStationExceptionEnum>,
    /// Toaster. A small, temperature controlled electric appliance with rectangular slots at the top that are lined with heated coils for the purpose of browning slices of bread products.
    
    pub toaster: Option<bool>,
    /// Toaster exception.
    #[serde(rename="toasterException")]
    
    pub toaster_exception: Option<LivingAreaEatingToasterExceptionEnum>,
}

impl client::Part for LivingAreaEating {}


/// Features in the living area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LivingAreaFeatures {
    /// Air conditioning. An electrical machine used to cool the temperature of the guestroom.
    #[serde(rename="airConditioning")]
    
    pub air_conditioning: Option<bool>,
    /// Air conditioning exception.
    #[serde(rename="airConditioningException")]
    
    pub air_conditioning_exception: Option<LivingAreaFeatureAirConditioningExceptionEnum>,
    /// Bathtub. A fixed plumbing feature set on the floor and consisting of a large container that accommodates the body of an adult for the purpose of seated bathing. Includes knobs or fixtures to control the temperature of the water, a faucet through which the water flows, and a drain that can be closed for filling and opened for draining.
    
    pub bathtub: Option<bool>,
    /// Bathtub exception.
    #[serde(rename="bathtubException")]
    
    pub bathtub_exception: Option<LivingAreaFeatureBathtubExceptionEnum>,
    /// Bidet. A plumbing fixture attached to a toilet or a low, fixed sink designed for the purpose of washing after toilet use.
    
    pub bidet: Option<bool>,
    /// Bidet exception.
    #[serde(rename="bidetException")]
    
    pub bidet_exception: Option<LivingAreaFeatureBidetExceptionEnum>,
    /// Dryer. An electrical machine designed to dry clothing.
    
    pub dryer: Option<bool>,
    /// Dryer exception.
    #[serde(rename="dryerException")]
    
    pub dryer_exception: Option<LivingAreaFeatureDryerExceptionEnum>,
    /// Electronic room key. A card coded by the check-in computer that is read by the lock on the hotel guestroom door to allow for entry.
    #[serde(rename="electronicRoomKey")]
    
    pub electronic_room_key: Option<bool>,
    /// Electronic room key exception.
    #[serde(rename="electronicRoomKeyException")]
    
    pub electronic_room_key_exception: Option<LivingAreaFeatureElectronicRoomKeyExceptionEnum>,
    /// Fireplace. A framed opening (aka hearth) at the base of a chimney in which logs or an electrical fire feature are burned to provide a relaxing ambiance or to heat the room. Often made of bricks or stone.
    
    pub fireplace: Option<bool>,
    /// Fireplace exception.
    #[serde(rename="fireplaceException")]
    
    pub fireplace_exception: Option<LivingAreaFeatureFireplaceExceptionEnum>,
    /// Hairdryer. A handheld electric appliance that blows temperature-controlled air for the purpose of drying wet hair. Can be mounted to a bathroom wall or a freestanding device stored in the guestroom's bathroom or closet.
    
    pub hairdryer: Option<bool>,
    /// Hairdryer exception.
    #[serde(rename="hairdryerException")]
    
    pub hairdryer_exception: Option<LivingAreaFeatureHairdryerExceptionEnum>,
    /// Heating. An electrical machine used to warm the temperature of the guestroom.
    
    pub heating: Option<bool>,
    /// Heating exception.
    #[serde(rename="heatingException")]
    
    pub heating_exception: Option<LivingAreaFeatureHeatingExceptionEnum>,
    /// In-unit safe. A strong fireproof cabinet with a programmable lock, used for the protected storage of valuables in a guestroom. Often built into a closet.
    #[serde(rename="inunitSafe")]
    
    pub inunit_safe: Option<bool>,
    /// In-unit safe exception.
    #[serde(rename="inunitSafeException")]
    
    pub inunit_safe_exception: Option<LivingAreaFeatureInunitSafeExceptionEnum>,
    /// In-unit Wifi available. Guests can wirelessly connect to the Internet in the guestroom. Can be free or for a fee.
    #[serde(rename="inunitWifiAvailable")]
    
    pub inunit_wifi_available: Option<bool>,
    /// In-unit Wifi available exception.
    #[serde(rename="inunitWifiAvailableException")]
    
    pub inunit_wifi_available_exception: Option<LivingAreaFeatureInunitWifiAvailableExceptionEnum>,
    /// Ironing equipment. A device, usually with a flat metal base, that is heated to smooth, finish, or press clothes and a flat, padded, cloth-covered surface on which the clothes are worked.
    #[serde(rename="ironingEquipment")]
    
    pub ironing_equipment: Option<bool>,
    /// Ironing equipment exception.
    #[serde(rename="ironingEquipmentException")]
    
    pub ironing_equipment_exception: Option<LivingAreaFeatureIroningEquipmentExceptionEnum>,
    /// Pay per view movies. Televisions with channels that offer films that can be viewed for a fee, and have an interface to allow the viewer to accept the terms and approve payment.
    #[serde(rename="payPerViewMovies")]
    
    pub pay_per_view_movies: Option<bool>,
    /// Pay per view movies exception.
    #[serde(rename="payPerViewMoviesException")]
    
    pub pay_per_view_movies_exception: Option<LivingAreaFeaturePayPerViewMoviesExceptionEnum>,
    /// Private bathroom. A bathroom designated for the express use of the guests staying in a specific guestroom.
    #[serde(rename="privateBathroom")]
    
    pub private_bathroom: Option<bool>,
    /// Private bathroom exception.
    #[serde(rename="privateBathroomException")]
    
    pub private_bathroom_exception: Option<LivingAreaFeaturePrivateBathroomExceptionEnum>,
    /// Shower. A fixed plumbing fixture for standing bathing that features a tall spray spout or faucet through which water flows, a knob or knobs that control the water's temperature, and a drain in the floor.
    
    pub shower: Option<bool>,
    /// Shower exception.
    #[serde(rename="showerException")]
    
    pub shower_exception: Option<LivingAreaFeatureShowerExceptionEnum>,
    /// Toilet. A fixed bathroom feature connected to a sewer or septic system and consisting of a water-flushed bowl with a seat, as well as a device that elicites the water-flushing action. Used for the process and disposal of human waste.
    
    pub toilet: Option<bool>,
    /// Toilet exception.
    #[serde(rename="toiletException")]
    
    pub toilet_exception: Option<LivingAreaFeatureToiletExceptionEnum>,
    /// TV. A television is available in the guestroom.
    
    pub tv: Option<bool>,
    /// TV casting. A television equipped with a device through which the video entertainment accessed on a personal computer, phone or tablet can be wirelessly delivered to and viewed on the guestroom's television.
    #[serde(rename="tvCasting")]
    
    pub tv_casting: Option<bool>,
    /// TV exception.
    #[serde(rename="tvCastingException")]
    
    pub tv_casting_exception: Option<LivingAreaFeatureTvCastingExceptionEnum>,
    /// TV exception.
    #[serde(rename="tvException")]
    
    pub tv_exception: Option<LivingAreaFeatureTvExceptionEnum>,
    /// TV streaming. Televisions that embed a range of web-based apps to allow for watching media from those apps.
    #[serde(rename="tvStreaming")]
    
    pub tv_streaming: Option<bool>,
    /// TV streaming exception.
    #[serde(rename="tvStreamingException")]
    
    pub tv_streaming_exception: Option<LivingAreaFeatureTvStreamingExceptionEnum>,
    /// Universal power adapters. A power supply for electronic devices which plugs into a wall for the purpose of converting AC to a single DC voltage. Also know as AC adapter or charger.
    #[serde(rename="universalPowerAdapters")]
    
    pub universal_power_adapters: Option<bool>,
    /// Universal power adapters exception.
    #[serde(rename="universalPowerAdaptersException")]
    
    pub universal_power_adapters_exception: Option<LivingAreaFeatureUniversalPowerAdaptersExceptionEnum>,
    /// Washer. An electrical machine connected to a running water source designed to launder clothing.
    
    pub washer: Option<bool>,
    /// Washer exception.
    #[serde(rename="washerException")]
    
    pub washer_exception: Option<LivingAreaFeatureWasherExceptionEnum>,
}

impl client::Part for LivingAreaFeatures {}


/// Information about the layout of the living area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LivingAreaLayout {
    /// Balcony. An outdoor platform attached to a building and surrounded by a short wall, fence or other safety railing. The balcony is accessed through a door in a guestroom or suite and is for use by the guest staying in that room. May or may not include seating or outdoor furniture. Is not located on the ground floor. Also lanai.
    
    pub balcony: Option<bool>,
    /// Balcony exception.
    #[serde(rename="balconyException")]
    
    pub balcony_exception: Option<LivingAreaLayoutBalconyExceptionEnum>,
    /// Living area sq meters. The measurement in meters of the area of a guestroom's living space.
    #[serde(rename="livingAreaSqMeters")]
    
    pub living_area_sq_meters: Option<f32>,
    /// Living area sq meters exception.
    #[serde(rename="livingAreaSqMetersException")]
    
    pub living_area_sq_meters_exception: Option<LivingAreaLayoutLivingAreaSqMetersExceptionEnum>,
    /// Loft. A three-walled upper area accessed by stairs or a ladder that overlooks the lower area of a room.
    
    pub loft: Option<bool>,
    /// Loft exception.
    #[serde(rename="loftException")]
    
    pub loft_exception: Option<LivingAreaLayoutLoftExceptionEnum>,
    /// Non smoking. A guestroom in which the smoking of cigarettes, cigars and pipes is prohibited.
    #[serde(rename="nonSmoking")]
    
    pub non_smoking: Option<bool>,
    /// Non smoking exception.
    #[serde(rename="nonSmokingException")]
    
    pub non_smoking_exception: Option<LivingAreaLayoutNonSmokingExceptionEnum>,
    /// Patio. A paved, outdoor area with seating attached to and accessed through a ground-floor guestroom for use by the occupants of the guestroom.
    
    pub patio: Option<bool>,
    /// Patio exception.
    #[serde(rename="patioException")]
    
    pub patio_exception: Option<LivingAreaLayoutPatioExceptionEnum>,
    /// Stairs. There are steps leading from one level or story to another in the unit.
    
    pub stairs: Option<bool>,
    /// Stairs exception.
    #[serde(rename="stairsException")]
    
    pub stairs_exception: Option<LivingAreaLayoutStairsExceptionEnum>,
}

impl client::Part for LivingAreaLayout {}


/// Information about sleeping features in the living area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LivingAreaSleeping {
    /// Beds count. The number of permanent beds present in a guestroom. Does not include rollaway beds, cribs or sofabeds.
    #[serde(rename="bedsCount")]
    
    pub beds_count: Option<i32>,
    /// Beds count exception.
    #[serde(rename="bedsCountException")]
    
    pub beds_count_exception: Option<LivingAreaSleepingBedsCountExceptionEnum>,
    /// Bunk beds count. The number of furniture pieces in which one framed mattress is fixed directly above another by means of a physical frame. This allows one person(s) to sleep in the bottom bunk and one person(s) to sleep in the top bunk. Also known as double decker bed.
    #[serde(rename="bunkBedsCount")]
    
    pub bunk_beds_count: Option<i32>,
    /// Bunk beds count exception.
    #[serde(rename="bunkBedsCountException")]
    
    pub bunk_beds_count_exception: Option<LivingAreaSleepingBunkBedsCountExceptionEnum>,
    /// Cribs count. The number of small beds for an infant or toddler that the guestroom can obtain. The bed is surrounded by a high railing to prevent the child from falling or climbing out of the bed
    #[serde(rename="cribsCount")]
    
    pub cribs_count: Option<i32>,
    /// Cribs count exception.
    #[serde(rename="cribsCountException")]
    
    pub cribs_count_exception: Option<LivingAreaSleepingCribsCountExceptionEnum>,
    /// Double beds count. The number of medium beds measuring 53"W x 75"L (135cm x 191cm). Also known as full size bed.
    #[serde(rename="doubleBedsCount")]
    
    pub double_beds_count: Option<i32>,
    /// Double beds count exception.
    #[serde(rename="doubleBedsCountException")]
    
    pub double_beds_count_exception: Option<LivingAreaSleepingDoubleBedsCountExceptionEnum>,
    /// Feather pillows. The option for guests to obtain bed pillows that are stuffed with the feathers and down of ducks or geese.
    #[serde(rename="featherPillows")]
    
    pub feather_pillows: Option<bool>,
    /// Feather pillows exception.
    #[serde(rename="featherPillowsException")]
    
    pub feather_pillows_exception: Option<LivingAreaSleepingFeatherPillowsExceptionEnum>,
    /// Hypoallergenic bedding. Bedding such as linens, pillows, mattress covers and/or mattresses that are made of materials known to be resistant to allergens such as mold, dust and dander.
    #[serde(rename="hypoallergenicBedding")]
    
    pub hypoallergenic_bedding: Option<bool>,
    /// Hypoallergenic bedding exception.
    #[serde(rename="hypoallergenicBeddingException")]
    
    pub hypoallergenic_bedding_exception: Option<LivingAreaSleepingHypoallergenicBeddingExceptionEnum>,
    /// King beds count. The number of large beds measuring 76"W x 80"L (193cm x 102cm). Most often meant to accompany two people. Includes California king and super king.
    #[serde(rename="kingBedsCount")]
    
    pub king_beds_count: Option<i32>,
    /// King beds count exception.
    #[serde(rename="kingBedsCountException")]
    
    pub king_beds_count_exception: Option<LivingAreaSleepingKingBedsCountExceptionEnum>,
    /// Memory foam pillows. The option for guests to obtain bed pillows that are stuffed with a man-made foam that responds to body heat by conforming to the body closely, and then recovers its shape when the pillow cools down.
    #[serde(rename="memoryFoamPillows")]
    
    pub memory_foam_pillows: Option<bool>,
    /// Memory foam pillows exception.
    #[serde(rename="memoryFoamPillowsException")]
    
    pub memory_foam_pillows_exception: Option<LivingAreaSleepingMemoryFoamPillowsExceptionEnum>,
    /// Other beds count. The number of beds that are not standard mattress and boxspring setups such as Japanese tatami mats, trundle beds, air mattresses and cots.
    #[serde(rename="otherBedsCount")]
    
    pub other_beds_count: Option<i32>,
    /// Other beds count exception.
    #[serde(rename="otherBedsCountException")]
    
    pub other_beds_count_exception: Option<LivingAreaSleepingOtherBedsCountExceptionEnum>,
    /// Queen beds count. The number of medium-large beds measuring 60"W x 80"L (152cm x 102cm).
    #[serde(rename="queenBedsCount")]
    
    pub queen_beds_count: Option<i32>,
    /// Queen beds count exception.
    #[serde(rename="queenBedsCountException")]
    
    pub queen_beds_count_exception: Option<LivingAreaSleepingQueenBedsCountExceptionEnum>,
    /// Roll away beds count. The number of mattresses on wheeled frames that can be folded in half and rolled away for easy storage that the guestroom can obtain upon request.
    #[serde(rename="rollAwayBedsCount")]
    
    pub roll_away_beds_count: Option<i32>,
    /// Roll away beds count exception.
    #[serde(rename="rollAwayBedsCountException")]
    
    pub roll_away_beds_count_exception: Option<LivingAreaSleepingRollAwayBedsCountExceptionEnum>,
    /// Single or twin count beds. The number of smaller beds measuring 38"W x 75"L (97cm x 191cm) that can accommodate one adult.
    #[serde(rename="singleOrTwinBedsCount")]
    
    pub single_or_twin_beds_count: Option<i32>,
    /// Single or twin beds count exception.
    #[serde(rename="singleOrTwinBedsCountException")]
    
    pub single_or_twin_beds_count_exception: Option<LivingAreaSleepingSingleOrTwinBedsCountExceptionEnum>,
    /// Sofa beds count. The number of specially designed sofas that can be made to serve as a bed by lowering its hinged upholstered back to horizontal position or by pulling out a concealed mattress.
    #[serde(rename="sofaBedsCount")]
    
    pub sofa_beds_count: Option<i32>,
    /// Sofa beds count exception.
    #[serde(rename="sofaBedsCountException")]
    
    pub sofa_beds_count_exception: Option<LivingAreaSleepingSofaBedsCountExceptionEnum>,
    /// Synthetic pillows. The option for guests to obtain bed pillows stuffed with polyester material crafted to reproduce the feel of a pillow stuffed with down and feathers.
    #[serde(rename="syntheticPillows")]
    
    pub synthetic_pillows: Option<bool>,
    /// Synthetic pillows exception.
    #[serde(rename="syntheticPillowsException")]
    
    pub synthetic_pillows_exception: Option<LivingAreaSleepingSyntheticPillowsExceptionEnum>,
}

impl client::Part for LivingAreaSleeping {}


/// Lodging of a location that provides accomodations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get lodging locations](LocationGetLodgingCall) (response)
/// * [update lodging locations](LocationUpdateLodgingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Lodging {
    /// Physical adaptations made to the property in consideration of varying levels of human physical ability.
    
    pub accessibility: Option<Accessibility>,
    /// Amenities and features related to leisure and play.
    
    pub activities: Option<Activities>,
    /// Output only. All units on the property have at least these attributes.
    #[serde(rename="allUnits")]
    
    pub all_units: Option<GuestUnitFeatures>,
    /// Features of the property of specific interest to the business traveler.
    
    pub business: Option<Business>,
    /// Features of the shared living areas available in this Lodging.
    #[serde(rename="commonLivingArea")]
    
    pub common_living_area: Option<LivingArea>,
    /// The ways in which the property provides guests with the ability to access the internet.
    
    pub connectivity: Option<Connectivity>,
    /// Services and amenities for families and young guests.
    
    pub families: Option<Families>,
    /// Meals, snacks, and beverages available at the property.
    #[serde(rename="foodAndDrink")]
    
    pub food_and_drink: Option<FoodAndDrink>,
    /// Individual GuestUnitTypes that are available in this Lodging.
    #[serde(rename="guestUnits")]
    
    pub guest_units: Option<Vec<GuestUnitType>>,
    /// Health and safety measures implemented by the hotel during COVID-19.
    #[serde(rename="healthAndSafety")]
    
    pub health_and_safety: Option<HealthAndSafety>,
    /// Conveniences provided in guest units to facilitate an easier, more comfortable stay.
    
    pub housekeeping: Option<Housekeeping>,
    /// Required. Metadata for the lodging.
    
    pub metadata: Option<LodgingMetadata>,
    /// Required. Google identifier for this location in the form: `locations/{location_id}/lodging`
    
    pub name: Option<String>,
    /// Parking options at the property.
    
    pub parking: Option<Parking>,
    /// Policies regarding guest-owned animals.
    
    pub pets: Option<Pets>,
    /// Property rules that impact guests.
    
    pub policies: Option<Policies>,
    /// Swimming pool or recreational water facilities available at the hotel.
    
    pub pools: Option<Pools>,
    /// General factual information about the property's physical structure and important dates.
    
    pub property: Option<Property>,
    /// Conveniences or help provided by the property to facilitate an easier, more comfortable stay.
    
    pub services: Option<Services>,
    /// Output only. Some units on the property have as much as these attributes.
    #[serde(rename="someUnits")]
    
    pub some_units: Option<GuestUnitFeatures>,
    /// Sustainability practices implemented at the hotel.
    
    pub sustainability: Option<Sustainability>,
    /// Vehicles or vehicular services facilitated or owned by the property.
    
    pub transportation: Option<Transportation>,
    /// Guest facilities at the property to promote or maintain health, beauty, and fitness.
    
    pub wellness: Option<Wellness>,
}

impl client::RequestValue for Lodging {}
impl client::ResponseResult for Lodging {}


/// Metadata for the Lodging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LodgingMetadata {
    /// Required. The latest time at which the Lodging data is asserted to be true in the real world. This is not necessarily the time at which the request is made.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for LodgingMetadata {}


/// Minimized contact measures implemented by the hotel during COVID-19.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MinimizedContact {
    /// No-contact check-in and check-out.
    #[serde(rename="contactlessCheckinCheckout")]
    
    pub contactless_checkin_checkout: Option<bool>,
    /// Contactless check-in check-out exception.
    #[serde(rename="contactlessCheckinCheckoutException")]
    
    pub contactless_checkin_checkout_exception: Option<MinimizedContactContactlessCheckinCheckoutExceptionEnum>,
    /// Keyless mobile entry to guest rooms.
    #[serde(rename="digitalGuestRoomKeys")]
    
    pub digital_guest_room_keys: Option<bool>,
    /// Digital guest room keys exception.
    #[serde(rename="digitalGuestRoomKeysException")]
    
    pub digital_guest_room_keys_exception: Option<MinimizedContactDigitalGuestRoomKeysExceptionEnum>,
    /// Housekeeping scheduled by request only.
    #[serde(rename="housekeepingScheduledRequestOnly")]
    
    pub housekeeping_scheduled_request_only: Option<bool>,
    /// Housekeeping scheduled request only exception.
    #[serde(rename="housekeepingScheduledRequestOnlyException")]
    
    pub housekeeping_scheduled_request_only_exception: Option<MinimizedContactHousekeepingScheduledRequestOnlyExceptionEnum>,
    /// High-touch items, such as magazines, removed from common areas.
    #[serde(rename="noHighTouchItemsCommonAreas")]
    
    pub no_high_touch_items_common_areas: Option<bool>,
    /// No high touch items common areas exception.
    #[serde(rename="noHighTouchItemsCommonAreasException")]
    
    pub no_high_touch_items_common_areas_exception: Option<MinimizedContactNoHighTouchItemsCommonAreasExceptionEnum>,
    /// High-touch items, such as decorative pillows, removed from guest rooms.
    #[serde(rename="noHighTouchItemsGuestRooms")]
    
    pub no_high_touch_items_guest_rooms: Option<bool>,
    /// No high touch items guest rooms exception.
    #[serde(rename="noHighTouchItemsGuestRoomsException")]
    
    pub no_high_touch_items_guest_rooms_exception: Option<MinimizedContactNoHighTouchItemsGuestRoomsExceptionEnum>,
    /// Plastic key cards are disinfected or discarded.
    #[serde(rename="plasticKeycardsDisinfected")]
    
    pub plastic_keycards_disinfected: Option<bool>,
    /// Plastic keycards disinfected exception.
    #[serde(rename="plasticKeycardsDisinfectedException")]
    
    pub plastic_keycards_disinfected_exception: Option<MinimizedContactPlasticKeycardsDisinfectedExceptionEnum>,
    /// Buffer maintained between room bookings.
    #[serde(rename="roomBookingsBuffer")]
    
    pub room_bookings_buffer: Option<bool>,
    /// Room bookings buffer exception.
    #[serde(rename="roomBookingsBufferException")]
    
    pub room_bookings_buffer_exception: Option<MinimizedContactRoomBookingsBufferExceptionEnum>,
}

impl client::Part for MinimizedContact {}


/// Parking options at the property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Parking {
    /// Electric car charging stations. Electric power stations, usually located outdoors, into which guests plug their electric cars to receive a charge.
    #[serde(rename="electricCarChargingStations")]
    
    pub electric_car_charging_stations: Option<bool>,
    /// Electric car charging stations exception.
    #[serde(rename="electricCarChargingStationsException")]
    
    pub electric_car_charging_stations_exception: Option<ParkingElectricCarChargingStationsExceptionEnum>,
    /// Free parking. The hotel allows the cars of guests to be parked for free. Parking facility may be an outdoor lot or an indoor garage, but must be onsite. Nearby parking does not apply. Parking may be performed by the guest or by hotel staff. Free parking must be available to all guests (limited conditions does not apply).
    #[serde(rename="freeParking")]
    
    pub free_parking: Option<bool>,
    /// Free parking exception.
    #[serde(rename="freeParkingException")]
    
    pub free_parking_exception: Option<ParkingFreeParkingExceptionEnum>,
    /// Free self parking. Guests park their own cars for free. Parking facility may be an outdoor lot or an indoor garage, but must be onsite. Nearby parking does not apply.
    #[serde(rename="freeSelfParking")]
    
    pub free_self_parking: Option<bool>,
    /// Free self parking exception.
    #[serde(rename="freeSelfParkingException")]
    
    pub free_self_parking_exception: Option<ParkingFreeSelfParkingExceptionEnum>,
    /// Free valet parking. Hotel staff member parks the cars of guests. Parking with this service is free.
    #[serde(rename="freeValetParking")]
    
    pub free_valet_parking: Option<bool>,
    /// Free valet parking exception.
    #[serde(rename="freeValetParkingException")]
    
    pub free_valet_parking_exception: Option<ParkingFreeValetParkingExceptionEnum>,
    /// Parking available. The hotel allows the cars of guests to be parked. Can be free or for a fee. Parking facility may be an outdoor lot or an indoor garage, but must be onsite. Nearby parking does not apply. Parking may be performed by the guest or by hotel staff.
    #[serde(rename="parkingAvailable")]
    
    pub parking_available: Option<bool>,
    /// Parking available exception.
    #[serde(rename="parkingAvailableException")]
    
    pub parking_available_exception: Option<ParkingParkingAvailableExceptionEnum>,
    /// Self parking available. Guests park their own cars. Parking facility may be an outdoor lot or an indoor garage, but must be onsite. Nearby parking does not apply. Can be free or for a fee.
    #[serde(rename="selfParkingAvailable")]
    
    pub self_parking_available: Option<bool>,
    /// Self parking available exception.
    #[serde(rename="selfParkingAvailableException")]
    
    pub self_parking_available_exception: Option<ParkingSelfParkingAvailableExceptionEnum>,
    /// Valet parking available. Hotel staff member parks the cars of guests. Parking with this service can be free or for a fee.
    #[serde(rename="valetParkingAvailable")]
    
    pub valet_parking_available: Option<bool>,
    /// Valet parking available exception.
    #[serde(rename="valetParkingAvailableException")]
    
    pub valet_parking_available_exception: Option<ParkingValetParkingAvailableExceptionEnum>,
}

impl client::Part for Parking {}


/// Forms of payment accepted at the property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PaymentOptions {
    /// Cash. The hotel accepts payment by paper/coin currency.
    
    pub cash: Option<bool>,
    /// Cash exception.
    #[serde(rename="cashException")]
    
    pub cash_exception: Option<PaymentOptionCashExceptionEnum>,
    /// Cheque. The hotel accepts a printed document issued by the guest's bank in the guest's name as a form of payment.
    
    pub cheque: Option<bool>,
    /// Cheque exception.
    #[serde(rename="chequeException")]
    
    pub cheque_exception: Option<PaymentOptionChequeExceptionEnum>,
    /// Credit card. The hotel accepts payment by a card issued by a bank or credit card company. Also known as charge card, debit card, bank card, or charge plate.
    #[serde(rename="creditCard")]
    
    pub credit_card: Option<bool>,
    /// Credit card exception.
    #[serde(rename="creditCardException")]
    
    pub credit_card_exception: Option<PaymentOptionCreditCardExceptionEnum>,
    /// Debit card. The hotel accepts a bank-issued card that immediately deducts the charged funds from the guest's bank account upon processing.
    #[serde(rename="debitCard")]
    
    pub debit_card: Option<bool>,
    /// Debit card exception.
    #[serde(rename="debitCardException")]
    
    pub debit_card_exception: Option<PaymentOptionDebitCardExceptionEnum>,
    /// Mobile nfc. The hotel has the compatible computer hardware terminal that reads and charges a payment app on the guest's smartphone without requiring the two devices to make physical contact. Also known as Apple Pay, Google Pay, Samsung Pay.
    #[serde(rename="mobileNfc")]
    
    pub mobile_nfc: Option<bool>,
    /// Mobile nfc exception.
    #[serde(rename="mobileNfcException")]
    
    pub mobile_nfc_exception: Option<PaymentOptionMobileNfcExceptionEnum>,
}

impl client::Part for PaymentOptions {}


/// Personal protection measures implemented by the hotel during COVID-19.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonalProtection {
    /// Hand-sanitizer and/or sanitizing wipes are offered in common areas.
    #[serde(rename="commonAreasOfferSanitizingItems")]
    
    pub common_areas_offer_sanitizing_items: Option<bool>,
    /// Common areas offer sanitizing items exception.
    #[serde(rename="commonAreasOfferSanitizingItemsException")]
    
    pub common_areas_offer_sanitizing_items_exception: Option<PersonalProtectionCommonAreasOfferSanitizingItemsExceptionEnum>,
    /// Masks required on the property.
    #[serde(rename="faceMaskRequired")]
    
    pub face_mask_required: Option<bool>,
    /// Face mask required exception.
    #[serde(rename="faceMaskRequiredException")]
    
    pub face_mask_required_exception: Option<PersonalProtectionFaceMaskRequiredExceptionEnum>,
    /// In-room hygiene kits with masks, hand sanitizer, and/or antibacterial wipes.
    #[serde(rename="guestRoomHygieneKitsAvailable")]
    
    pub guest_room_hygiene_kits_available: Option<bool>,
    /// Guest room hygiene kits available exception.
    #[serde(rename="guestRoomHygieneKitsAvailableException")]
    
    pub guest_room_hygiene_kits_available_exception: Option<PersonalProtectionGuestRoomHygieneKitsAvailableExceptionEnum>,
    /// Masks and/or gloves available for guests.
    #[serde(rename="protectiveEquipmentAvailable")]
    
    pub protective_equipment_available: Option<bool>,
    /// Protective equipment available exception.
    #[serde(rename="protectiveEquipmentAvailableException")]
    
    pub protective_equipment_available_exception: Option<PersonalProtectionProtectiveEquipmentAvailableExceptionEnum>,
}

impl client::Part for PersonalProtection {}


/// Policies regarding guest-owned animals.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pets {
    /// Cats allowed. Domesticated felines are permitted at the property and allowed to stay in the guest room of their owner. May or may not require a fee.
    #[serde(rename="catsAllowed")]
    
    pub cats_allowed: Option<bool>,
    /// Cats allowed exception.
    #[serde(rename="catsAllowedException")]
    
    pub cats_allowed_exception: Option<PetCatsAllowedExceptionEnum>,
    /// Dogs allowed. Domesticated canines are permitted at the property and allowed to stay in the guest room of their owner. May or may not require a fee.
    #[serde(rename="dogsAllowed")]
    
    pub dogs_allowed: Option<bool>,
    /// Dogs allowed exception.
    #[serde(rename="dogsAllowedException")]
    
    pub dogs_allowed_exception: Option<PetDogsAllowedExceptionEnum>,
    /// Pets allowed. Household animals are allowed at the property and in the specific guest room of their owner. May or may not include dogs, cats, reptiles and/or fish. May or may not require a fee. Service animals are not considered to be pets, so not governed by this policy.
    #[serde(rename="petsAllowed")]
    
    pub pets_allowed: Option<bool>,
    /// Pets allowed exception.
    #[serde(rename="petsAllowedException")]
    
    pub pets_allowed_exception: Option<PetPetsAllowedExceptionEnum>,
    /// Pets allowed free. Household animals are allowed at the property and in the specific guest room of their owner for free. May or may not include dogs, cats, reptiles, and/or fish.
    #[serde(rename="petsAllowedFree")]
    
    pub pets_allowed_free: Option<bool>,
    /// Pets allowed free exception.
    #[serde(rename="petsAllowedFreeException")]
    
    pub pets_allowed_free_exception: Option<PetPetsAllowedFreeExceptionEnum>,
}

impl client::Part for Pets {}


/// Physical distancing measures implemented by the hotel during COVID-19.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PhysicalDistancing {
    /// Common areas arranged to maintain physical distancing.
    #[serde(rename="commonAreasPhysicalDistancingArranged")]
    
    pub common_areas_physical_distancing_arranged: Option<bool>,
    /// Common areas physical distancing arranged exception.
    #[serde(rename="commonAreasPhysicalDistancingArrangedException")]
    
    pub common_areas_physical_distancing_arranged_exception: Option<PhysicalDistancingCommonAreasPhysicalDistancingArrangedExceptionEnum>,
    /// Physical distancing required.
    #[serde(rename="physicalDistancingRequired")]
    
    pub physical_distancing_required: Option<bool>,
    /// Physical distancing required exception.
    #[serde(rename="physicalDistancingRequiredException")]
    
    pub physical_distancing_required_exception: Option<PhysicalDistancingPhysicalDistancingRequiredExceptionEnum>,
    /// Safety dividers at front desk and other locations.
    #[serde(rename="safetyDividers")]
    
    pub safety_dividers: Option<bool>,
    /// Safety dividers exception.
    #[serde(rename="safetyDividersException")]
    
    pub safety_dividers_exception: Option<PhysicalDistancingSafetyDividersExceptionEnum>,
    /// Guest occupancy limited within shared facilities.
    #[serde(rename="sharedAreasLimitedOccupancy")]
    
    pub shared_areas_limited_occupancy: Option<bool>,
    /// Shared areas limited occupancy exception.
    #[serde(rename="sharedAreasLimitedOccupancyException")]
    
    pub shared_areas_limited_occupancy_exception: Option<PhysicalDistancingSharedAreasLimitedOccupancyExceptionEnum>,
    /// Private spaces designated in spa and wellness areas.
    #[serde(rename="wellnessAreasHavePrivateSpaces")]
    
    pub wellness_areas_have_private_spaces: Option<bool>,
    /// Wellness areas have private spaces exception.
    #[serde(rename="wellnessAreasHavePrivateSpacesException")]
    
    pub wellness_areas_have_private_spaces_exception: Option<PhysicalDistancingWellnessAreasHavePrivateSpacesExceptionEnum>,
}

impl client::Part for PhysicalDistancing {}


/// Property rules that impact guests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policies {
    /// All inclusive available. The hotel offers a rate option that includes the cost of the room, meals, activities, and other amenities that might otherwise be charged separately.
    #[serde(rename="allInclusiveAvailable")]
    
    pub all_inclusive_available: Option<bool>,
    /// All inclusive available exception.
    #[serde(rename="allInclusiveAvailableException")]
    
    pub all_inclusive_available_exception: Option<PolicyAllInclusiveAvailableExceptionEnum>,
    /// All inclusive only. The only rate option offered by the hotel is a rate that includes the cost of the room, meals, activities and other amenities that might otherwise be charged separately.
    #[serde(rename="allInclusiveOnly")]
    
    pub all_inclusive_only: Option<bool>,
    /// All inclusive only exception.
    #[serde(rename="allInclusiveOnlyException")]
    
    pub all_inclusive_only_exception: Option<PolicyAllInclusiveOnlyExceptionEnum>,
    /// Check-in time. The time of the day at which the hotel begins providing guests access to their unit at the beginning of their stay.
    #[serde(rename="checkinTime")]
    
    pub checkin_time: Option<TimeOfDay>,
    /// Check-in time exception.
    #[serde(rename="checkinTimeException")]
    
    pub checkin_time_exception: Option<PolicyCheckinTimeExceptionEnum>,
    /// Check-out time. The time of the day on the last day of a guest's reserved stay at which the guest must vacate their room and settle their bill. Some hotels may offer late or early check out for a fee.
    #[serde(rename="checkoutTime")]
    
    pub checkout_time: Option<TimeOfDay>,
    /// Check-out time exception.
    #[serde(rename="checkoutTimeException")]
    
    pub checkout_time_exception: Option<PolicyCheckoutTimeExceptionEnum>,
    /// Kids stay free. The children of guests are allowed to stay in the room/suite of a parent or adult without an additional fee. The policy may or may not stipulate a limit of the child's age or the overall number of children allowed.
    #[serde(rename="kidsStayFree")]
    
    pub kids_stay_free: Option<bool>,
    /// Kids stay free exception.
    #[serde(rename="kidsStayFreeException")]
    
    pub kids_stay_free_exception: Option<PolicyKidsStayFreeExceptionEnum>,
    /// Max child age. The hotel allows children up to a certain age to stay in the room/suite of a parent or adult without an additional fee.
    #[serde(rename="maxChildAge")]
    
    pub max_child_age: Option<i32>,
    /// Max child age exception.
    #[serde(rename="maxChildAgeException")]
    
    pub max_child_age_exception: Option<PolicyMaxChildAgeExceptionEnum>,
    /// Max kids stay free count. The hotel allows a specific, defined number of children to stay in the room/suite of a parent or adult without an additional fee.
    #[serde(rename="maxKidsStayFreeCount")]
    
    pub max_kids_stay_free_count: Option<i32>,
    /// Max kids stay free count exception.
    #[serde(rename="maxKidsStayFreeCountException")]
    
    pub max_kids_stay_free_count_exception: Option<PolicyMaxKidsStayFreeCountExceptionEnum>,
    /// Forms of payment accepted at the property.
    #[serde(rename="paymentOptions")]
    
    pub payment_options: Option<PaymentOptions>,
    /// Smoke free property. Smoking is not allowed inside the building, on balconies, or in outside spaces. Hotels that offer a designated area for guests to smoke are not considered smoke-free properties.
    #[serde(rename="smokeFreeProperty")]
    
    pub smoke_free_property: Option<bool>,
    /// Smoke free property exception.
    #[serde(rename="smokeFreePropertyException")]
    
    pub smoke_free_property_exception: Option<PolicySmokeFreePropertyExceptionEnum>,
}

impl client::Part for Policies {}


/// Swimming pool or recreational water facilities available at the hotel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pools {
    /// Adult pool. A pool restricted for use by adults only. Can be indoors or outdoors.
    #[serde(rename="adultPool")]
    
    pub adult_pool: Option<bool>,
    /// Adult pool exception.
    #[serde(rename="adultPoolException")]
    
    pub adult_pool_exception: Option<PoolAdultPoolExceptionEnum>,
    /// Hot tub. A man-made pool containing bubbling water maintained at a higher temperature and circulated by aerating jets for the purpose of soaking, relaxation and hydrotherapy. Can be indoors or outdoors. Not used for active swimming. Also known as Jacuzzi. Hot tub must be in a common area where all guests can access it. Does not apply to room-specific hot tubs that are only accessible to guest occupying that room.
    #[serde(rename="hotTub")]
    
    pub hot_tub: Option<bool>,
    /// Hot tub exception.
    #[serde(rename="hotTubException")]
    
    pub hot_tub_exception: Option<PoolHotTubExceptionEnum>,
    /// Indoor pool. A pool located inside the hotel and available for guests to use for swimming and/or soaking. Use may or may not be restricted to adults and/or children.
    #[serde(rename="indoorPool")]
    
    pub indoor_pool: Option<bool>,
    /// Indoor pool exception.
    #[serde(rename="indoorPoolException")]
    
    pub indoor_pool_exception: Option<PoolIndoorPoolExceptionEnum>,
    /// Indoor pools count. The sum of all indoor pools at the hotel.
    #[serde(rename="indoorPoolsCount")]
    
    pub indoor_pools_count: Option<i32>,
    /// Indoor pools count exception.
    #[serde(rename="indoorPoolsCountException")]
    
    pub indoor_pools_count_exception: Option<PoolIndoorPoolsCountExceptionEnum>,
    /// Lazy river. A man-made pool or several interconnected recreational pools built to mimic the shape and current of a winding river where guests float in the water on inflated rubber tubes. Can be indoors or outdoors.
    #[serde(rename="lazyRiver")]
    
    pub lazy_river: Option<bool>,
    /// Lazy river exception.
    #[serde(rename="lazyRiverException")]
    
    pub lazy_river_exception: Option<PoolLazyRiverExceptionEnum>,
    /// Lifeguard. A trained member of the hotel staff stationed by the hotel's indoor or outdoor swimming area and responsible for the safety of swimming guests.
    
    pub lifeguard: Option<bool>,
    /// Lifeguard exception.
    #[serde(rename="lifeguardException")]
    
    pub lifeguard_exception: Option<PoolLifeguardExceptionEnum>,
    /// Outdoor pool. A pool located outside on the grounds of the hotel and available for guests to use for swimming, soaking or recreation. Use may or may not be restricted to adults and/or children.
    #[serde(rename="outdoorPool")]
    
    pub outdoor_pool: Option<bool>,
    /// Outdoor pool exception.
    #[serde(rename="outdoorPoolException")]
    
    pub outdoor_pool_exception: Option<PoolOutdoorPoolExceptionEnum>,
    /// Outdoor pools count. The sum of all outdoor pools at the hotel.
    #[serde(rename="outdoorPoolsCount")]
    
    pub outdoor_pools_count: Option<i32>,
    /// Outdoor pools count exception.
    #[serde(rename="outdoorPoolsCountException")]
    
    pub outdoor_pools_count_exception: Option<PoolOutdoorPoolsCountExceptionEnum>,
    /// Pool. The presence of a pool, either indoors or outdoors, for guests to use for swimming and/or soaking. Use may or may not be restricted to adults and/or children.
    
    pub pool: Option<bool>,
    /// Pool exception.
    #[serde(rename="poolException")]
    
    pub pool_exception: Option<PoolPoolExceptionEnum>,
    /// Pools count. The sum of all pools at the hotel.
    #[serde(rename="poolsCount")]
    
    pub pools_count: Option<i32>,
    /// Pools count exception.
    #[serde(rename="poolsCountException")]
    
    pub pools_count_exception: Option<PoolPoolsCountExceptionEnum>,
    /// Wading pool. A shallow pool designed for small children to play in. Can be indoors or outdoors. Also known as kiddie pool.
    #[serde(rename="wadingPool")]
    
    pub wading_pool: Option<bool>,
    /// Wading pool exception.
    #[serde(rename="wadingPoolException")]
    
    pub wading_pool_exception: Option<PoolWadingPoolExceptionEnum>,
    /// Water park. An aquatic recreation area with a large pool or series of pools that has features such as a water slide or tube, wavepool, fountains, rope swings, and/or obstacle course. Can be indoors or outdoors. Also known as adventure pool.
    #[serde(rename="waterPark")]
    
    pub water_park: Option<bool>,
    /// Water park exception.
    #[serde(rename="waterParkException")]
    
    pub water_park_exception: Option<PoolWaterParkExceptionEnum>,
    /// Waterslide. A continuously wetted chute positioned by an indoor or outdoor pool which people slide down into the water.
    
    pub waterslide: Option<bool>,
    /// Waterslide exception.
    #[serde(rename="waterslideException")]
    
    pub waterslide_exception: Option<PoolWaterslideExceptionEnum>,
    /// Wave pool. A large indoor or outdoor pool with a machine that produces water currents to mimic the ocean's crests.
    #[serde(rename="wavePool")]
    
    pub wave_pool: Option<bool>,
    /// Wave pool exception.
    #[serde(rename="wavePoolException")]
    
    pub wave_pool_exception: Option<PoolWavePoolExceptionEnum>,
}

impl client::Part for Pools {}


/// General factual information about the property's physical structure and important dates.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    /// Built year. The year that construction of the property was completed.
    #[serde(rename="builtYear")]
    
    pub built_year: Option<i32>,
    /// Built year exception.
    #[serde(rename="builtYearException")]
    
    pub built_year_exception: Option<PropertyBuiltYearExceptionEnum>,
    /// Floors count. The number of stories the building has from the ground floor to the top floor that are accessible to guests.
    #[serde(rename="floorsCount")]
    
    pub floors_count: Option<i32>,
    /// Floors count exception.
    #[serde(rename="floorsCountException")]
    
    pub floors_count_exception: Option<PropertyFloorsCountExceptionEnum>,
    /// Last renovated year. The year when the most recent renovation of the property was completed. Renovation may include all or any combination of the following: the units, the public spaces, the exterior, or the interior.
    #[serde(rename="lastRenovatedYear")]
    
    pub last_renovated_year: Option<i32>,
    /// Last renovated year exception.
    #[serde(rename="lastRenovatedYearException")]
    
    pub last_renovated_year_exception: Option<PropertyLastRenovatedYearExceptionEnum>,
    /// Rooms count. The total number of rooms and suites bookable by guests for an overnight stay. Does not include event space, public spaces, conference rooms, fitness rooms, business centers, spa, salon, restaurants/bars, or shops.
    #[serde(rename="roomsCount")]
    
    pub rooms_count: Option<i32>,
    /// Rooms count exception.
    #[serde(rename="roomsCountException")]
    
    pub rooms_count_exception: Option<PropertyRoomsCountExceptionEnum>,
}

impl client::Part for Property {}


/// Conveniences or help provided by the property to facilitate an easier, more comfortable stay.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Services {
    /// Baggage storage. A provision for guests to leave their bags at the hotel when they arrive for their stay before the official check-in time. May or may not apply for guests who wish to leave their bags after check-out and before departing the locale. Also known as bag dropoff.
    #[serde(rename="baggageStorage")]
    
    pub baggage_storage: Option<bool>,
    /// Baggage storage exception.
    #[serde(rename="baggageStorageException")]
    
    pub baggage_storage_exception: Option<ServiceBaggageStorageExceptionEnum>,
    /// Concierge. Hotel staff member(s) responsible for facilitating an easy, comfortable stay through making reservations for meals, sourcing theater tickets, arranging tours, finding a doctor, making recommendations, and answering questions.
    
    pub concierge: Option<bool>,
    /// Concierge exception.
    #[serde(rename="conciergeException")]
    
    pub concierge_exception: Option<ServiceConciergeExceptionEnum>,
    /// Convenience store. A shop at the hotel primarily selling snacks, drinks, non-prescription medicines, health and beauty aids, magazines and newspapers.
    #[serde(rename="convenienceStore")]
    
    pub convenience_store: Option<bool>,
    /// Convenience store exception.
    #[serde(rename="convenienceStoreException")]
    
    pub convenience_store_exception: Option<ServiceConvenienceStoreExceptionEnum>,
    /// Currency exchange. A staff member or automated machine tasked with the transaction of providing the native currency of the hotel's locale in exchange for the foreign currency provided by a guest.
    #[serde(rename="currencyExchange")]
    
    pub currency_exchange: Option<bool>,
    /// Currency exchange exception.
    #[serde(rename="currencyExchangeException")]
    
    pub currency_exchange_exception: Option<ServiceCurrencyExchangeExceptionEnum>,
    /// Elevator. A passenger elevator that transports guests from one story to another. Also known as lift.
    
    pub elevator: Option<bool>,
    /// Elevator exception.
    #[serde(rename="elevatorException")]
    
    pub elevator_exception: Option<ServiceElevatorExceptionEnum>,
    /// Front desk. A counter or desk in the lobby or the immediate interior of the hotel where a member of the staff greets guests and processes the information related to their stay (including check-in and check-out). May or may not be manned and open 24/7.
    #[serde(rename="frontDesk")]
    
    pub front_desk: Option<bool>,
    /// Front desk exception.
    #[serde(rename="frontDeskException")]
    
    pub front_desk_exception: Option<ServiceFrontDeskExceptionEnum>,
    /// Full service laundry. Laundry and dry cleaning facilitated and handled by the hotel on behalf of the guest. Does not include the provision for guests to do their own laundry in on-site machines.
    #[serde(rename="fullServiceLaundry")]
    
    pub full_service_laundry: Option<bool>,
    /// Full service laundry exception.
    #[serde(rename="fullServiceLaundryException")]
    
    pub full_service_laundry_exception: Option<ServiceFullServiceLaundryExceptionEnum>,
    /// Gift shop. An on-site store primarily selling souvenirs, mementos and other gift items. May or may not also sell sundries, magazines and newspapers, clothing, or snacks.
    #[serde(rename="giftShop")]
    
    pub gift_shop: Option<bool>,
    /// Gift shop exception.
    #[serde(rename="giftShopException")]
    
    pub gift_shop_exception: Option<ServiceGiftShopExceptionEnum>,
    /// Languages spoken by at least one staff member.
    #[serde(rename="languagesSpoken")]
    
    pub languages_spoken: Option<Vec<LanguageSpoken>>,
    /// Self service laundry. On-site clothes washers and dryers accessible to guests for the purpose of washing and drying their own clothes. May or may not require payment to use the machines.
    #[serde(rename="selfServiceLaundry")]
    
    pub self_service_laundry: Option<bool>,
    /// Self service laundry exception.
    #[serde(rename="selfServiceLaundryException")]
    
    pub self_service_laundry_exception: Option<ServiceSelfServiceLaundryExceptionEnum>,
    /// Social hour. A reception with complimentary soft drinks, tea, coffee, wine and/or cocktails in the afternoon or evening. Can be hosted by hotel staff or guests may serve themselves. Also known as wine hour. The availability of coffee/tea in the lobby throughout the day does not constitute a social or wine hour.
    #[serde(rename="socialHour")]
    
    pub social_hour: Option<bool>,
    /// Social hour exception.
    #[serde(rename="socialHourException")]
    
    pub social_hour_exception: Option<ServiceSocialHourExceptionEnum>,
    /// 24hr front desk. Front desk is staffed 24 hours a day.
    #[serde(rename="twentyFourHourFrontDesk")]
    
    pub twenty_four_hour_front_desk: Option<bool>,
    /// 24hr front desk exception.
    #[serde(rename="twentyFourHourFrontDeskException")]
    
    pub twenty_four_hour_front_desk_exception: Option<ServiceTwentyFourHourFrontDeskExceptionEnum>,
    /// Wake up calls. By direction of the guest, a hotel staff member will phone the guest unit at the requested hour. Also known as morning call.
    #[serde(rename="wakeUpCalls")]
    
    pub wake_up_calls: Option<bool>,
    /// Wake up calls exception.
    #[serde(rename="wakeUpCallsException")]
    
    pub wake_up_calls_exception: Option<ServiceWakeUpCallsExceptionEnum>,
}

impl client::Part for Services {}


/// Sustainability practices implemented at the hotel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sustainability {
    /// Energy efficiency practices implemented at the hotel.
    #[serde(rename="energyEfficiency")]
    
    pub energy_efficiency: Option<EnergyEfficiency>,
    /// Sustainability certifications the hotel has been awarded.
    #[serde(rename="sustainabilityCertifications")]
    
    pub sustainability_certifications: Option<SustainabilityCertifications>,
    /// Sustainable sourcing practices implemented at the hotel.
    #[serde(rename="sustainableSourcing")]
    
    pub sustainable_sourcing: Option<SustainableSourcing>,
    /// Waste reduction practices implemented at the hotel.
    #[serde(rename="wasteReduction")]
    
    pub waste_reduction: Option<WasteReduction>,
    /// Water conservation practices implemented at the hotel.
    #[serde(rename="waterConservation")]
    
    pub water_conservation: Option<WaterConservation>,
}

impl client::Part for Sustainability {}


/// Sustainability certifications the hotel has been awarded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SustainabilityCertifications {
    /// BREEAM certification.
    #[serde(rename="breeamCertification")]
    
    pub breeam_certification: Option<SustainabilityCertificationBreeamCertificationEnum>,
    /// BREEAM certification exception.
    #[serde(rename="breeamCertificationException")]
    
    pub breeam_certification_exception: Option<SustainabilityCertificationBreeamCertificationExceptionEnum>,
    /// The eco certificates awarded to the hotel.
    #[serde(rename="ecoCertifications")]
    
    pub eco_certifications: Option<Vec<EcoCertification>>,
    /// LEED certification. Deprecated: this field is no longer populated. LEED certification status is now provided directly by USGBC.
    #[serde(rename="leedCertification")]
    
    pub leed_certification: Option<SustainabilityCertificationLeedCertificationEnum>,
    /// LEED certification exception. Deprecated: this field is no longer populated. LEED certification status is now provided directly by USGBC.
    #[serde(rename="leedCertificationException")]
    
    pub leed_certification_exception: Option<SustainabilityCertificationLeedCertificationExceptionEnum>,
}

impl client::Part for SustainabilityCertifications {}


/// Sustainable sourcing practices implemented at the hotel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SustainableSourcing {
    /// Eco friendly toiletries. Soap, shampoo, lotion, and other toiletries provided for guests have a nationally or internationally recognized sustainability certification, such as USDA Organic, EU Organic, or cruelty-free.
    #[serde(rename="ecoFriendlyToiletries")]
    
    pub eco_friendly_toiletries: Option<bool>,
    /// Eco friendly toiletries exception.
    #[serde(rename="ecoFriendlyToiletriesException")]
    
    pub eco_friendly_toiletries_exception: Option<SustainableSourcingEcoFriendlyToiletriesExceptionEnum>,
    /// Locally sourced food and beverages. Property sources locally in order to lower the environmental footprint from reduced transportation and to stimulate the local economy. Products produced less than 62 miles from the establishment are normally considered as locally produced.
    #[serde(rename="locallySourcedFoodAndBeverages")]
    
    pub locally_sourced_food_and_beverages: Option<bool>,
    /// Locally sourced food and beverages exception.
    #[serde(rename="locallySourcedFoodAndBeveragesException")]
    
    pub locally_sourced_food_and_beverages_exception: Option<SustainableSourcingLocallySourcedFoodAndBeveragesExceptionEnum>,
    /// Organic cage free eggs. The property sources 100% certified organic and cage-free eggs (shell, liquid, and egg products). Cage-free means hens are able to walk, spread their wings and lay their eggs in nests).
    #[serde(rename="organicCageFreeEggs")]
    
    pub organic_cage_free_eggs: Option<bool>,
    /// Organic cage free eggs exception.
    #[serde(rename="organicCageFreeEggsException")]
    
    pub organic_cage_free_eggs_exception: Option<SustainableSourcingOrganicCageFreeEggsExceptionEnum>,
    /// Organic food and beverages. At least 25% of food and beverages, by spend, are certified organic. Organic means products that are certified to one of the organic standard listed in the IFOAM family of standards. Qualifying certifications include USDA Organic and EU Organic, among others.
    #[serde(rename="organicFoodAndBeverages")]
    
    pub organic_food_and_beverages: Option<bool>,
    /// Organic food and beverages exception.
    #[serde(rename="organicFoodAndBeveragesException")]
    
    pub organic_food_and_beverages_exception: Option<SustainableSourcingOrganicFoodAndBeveragesExceptionEnum>,
    /// Responsible purchasing policy. The property has a responsible procurement policy in place. Responsible means integration of social, ethical, and/or environmental performance factors into the procurement process when selecting suppliers.
    #[serde(rename="responsiblePurchasingPolicy")]
    
    pub responsible_purchasing_policy: Option<bool>,
    /// Responsible purchasing policy exception.
    #[serde(rename="responsiblePurchasingPolicyException")]
    
    pub responsible_purchasing_policy_exception: Option<SustainableSourcingResponsiblePurchasingPolicyExceptionEnum>,
    /// Responsibly sources seafood. The property does not source seafood from the Monterey Bay Aquarium Seafood Watch "avoid" list, and must sustainably source seafood listed as "good alternative," "eco-certified," and "best choice". The property has a policy outlining a commitment to source Marine Stewardship Council (MSC) and/or Aquaculture Stewardship Council (ASC) Chain of Custody certified seafood.
    #[serde(rename="responsiblySourcesSeafood")]
    
    pub responsibly_sources_seafood: Option<bool>,
    /// Responsibly sources seafood exception.
    #[serde(rename="responsiblySourcesSeafoodException")]
    
    pub responsibly_sources_seafood_exception: Option<SustainableSourcingResponsiblySourcesSeafoodExceptionEnum>,
    /// Vegan meals. The property provides vegan menu options for guests. Vegan food does not contain animal products or byproducts.
    #[serde(rename="veganMeals")]
    
    pub vegan_meals: Option<bool>,
    /// Vegan meals exception.
    #[serde(rename="veganMealsException")]
    
    pub vegan_meals_exception: Option<SustainableSourcingVeganMealsExceptionEnum>,
    /// Vegetarian meals. The property provides vegetarian menu options for guests. Vegetarian food does not contain meat, poultry, fish, or seafood.
    #[serde(rename="vegetarianMeals")]
    
    pub vegetarian_meals: Option<bool>,
    /// Vegetarian meals exception.
    #[serde(rename="vegetarianMealsException")]
    
    pub vegetarian_meals_exception: Option<SustainableSourcingVegetarianMealsExceptionEnum>,
}

impl client::Part for SustainableSourcing {}


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// Vehicles or vehicular services facilitated or owned by the property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Transportation {
    /// Airport shuttle. The hotel provides guests with a chauffeured van or bus to and from the airport. Can be free or for a fee. Guests may share the vehicle with other guests unknown to them. Applies if the hotel has a third-party shuttle service (office/desk etc.) within the hotel. As long as hotel provides this service, it doesn't matter if it's directly with them or a third party they work with. Does not apply if guest has to coordinate with an entity outside/other than the hotel.
    #[serde(rename="airportShuttle")]
    
    pub airport_shuttle: Option<bool>,
    /// Airport shuttle exception.
    #[serde(rename="airportShuttleException")]
    
    pub airport_shuttle_exception: Option<TransportationAirportShuttleExceptionEnum>,
    /// Car rental on property. A branch of a rental car company with a processing desk in the hotel. Available cars for rent may be awaiting at the hotel or in a nearby lot.
    #[serde(rename="carRentalOnProperty")]
    
    pub car_rental_on_property: Option<bool>,
    /// Car rental on property exception.
    #[serde(rename="carRentalOnPropertyException")]
    
    pub car_rental_on_property_exception: Option<TransportationCarRentalOnPropertyExceptionEnum>,
    /// Free airport shuttle. Airport shuttle is free to guests. Must be free to all guests without any conditions.
    #[serde(rename="freeAirportShuttle")]
    
    pub free_airport_shuttle: Option<bool>,
    /// Free airport shuttle exception.
    #[serde(rename="freeAirportShuttleException")]
    
    pub free_airport_shuttle_exception: Option<TransportationFreeAirportShuttleExceptionEnum>,
    /// Free private car service. Private chauffeured car service is free to guests.
    #[serde(rename="freePrivateCarService")]
    
    pub free_private_car_service: Option<bool>,
    /// Free private car service exception.
    #[serde(rename="freePrivateCarServiceException")]
    
    pub free_private_car_service_exception: Option<TransportationFreePrivateCarServiceExceptionEnum>,
    /// Local shuttle. A car, van or bus provided by the hotel to transport guests to destinations within a specified range of distance around the hotel. Usually shopping and/or convention centers, downtown districts, or beaches. Can be free or for a fee.
    #[serde(rename="localShuttle")]
    
    pub local_shuttle: Option<bool>,
    /// Local shuttle exception.
    #[serde(rename="localShuttleException")]
    
    pub local_shuttle_exception: Option<TransportationLocalShuttleExceptionEnum>,
    /// Private car service. Hotel provides a private chauffeured car to transport guests to destinations. Passengers in the car are either alone or are known to one another and have requested the car together. Service can be free or for a fee and travel distance is usually limited to a specific range. Not a taxi.
    #[serde(rename="privateCarService")]
    
    pub private_car_service: Option<bool>,
    /// Private car service exception.
    #[serde(rename="privateCarServiceException")]
    
    pub private_car_service_exception: Option<TransportationPrivateCarServiceExceptionEnum>,
    /// Transfer. Hotel provides a shuttle service or car service to take guests to and from the nearest airport or train station. Can be free or for a fee. Guests may share the vehicle with other guests unknown to them.
    
    pub transfer: Option<bool>,
    /// Transfer exception.
    #[serde(rename="transferException")]
    
    pub transfer_exception: Option<TransportationTransferExceptionEnum>,
}

impl client::Part for Transportation {}


/// Views available from the guest unit itself.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViewsFromUnit {
    /// Beach view. A guestroom that features a window through which guests can see the beach.
    #[serde(rename="beachView")]
    
    pub beach_view: Option<bool>,
    /// Beach view exception.
    #[serde(rename="beachViewException")]
    
    pub beach_view_exception: Option<ViewsFromUnitBeachViewExceptionEnum>,
    /// City view. A guestroom that features a window through which guests can see the buildings, parks and/or streets of the city.
    #[serde(rename="cityView")]
    
    pub city_view: Option<bool>,
    /// City view exception.
    #[serde(rename="cityViewException")]
    
    pub city_view_exception: Option<ViewsFromUnitCityViewExceptionEnum>,
    /// Garden view. A guestroom that features a window through which guests can see a garden.
    #[serde(rename="gardenView")]
    
    pub garden_view: Option<bool>,
    /// Garden view exception.
    #[serde(rename="gardenViewException")]
    
    pub garden_view_exception: Option<ViewsFromUnitGardenViewExceptionEnum>,
    /// Lake view.
    #[serde(rename="lakeView")]
    
    pub lake_view: Option<bool>,
    /// Lake view exception.
    #[serde(rename="lakeViewException")]
    
    pub lake_view_exception: Option<ViewsFromUnitLakeViewExceptionEnum>,
    /// Landmark view. A guestroom that features a window through which guests can see a landmark such as the countryside, a golf course, the forest, a park, a rain forst, a mountain or a slope.
    #[serde(rename="landmarkView")]
    
    pub landmark_view: Option<bool>,
    /// Landmark view exception.
    #[serde(rename="landmarkViewException")]
    
    pub landmark_view_exception: Option<ViewsFromUnitLandmarkViewExceptionEnum>,
    /// Ocean view. A guestroom that features a window through which guests can see the ocean.
    #[serde(rename="oceanView")]
    
    pub ocean_view: Option<bool>,
    /// Ocean view exception.
    #[serde(rename="oceanViewException")]
    
    pub ocean_view_exception: Option<ViewsFromUnitOceanViewExceptionEnum>,
    /// Pool view. A guestroom that features a window through which guests can see the hotel's swimming pool.
    #[serde(rename="poolView")]
    
    pub pool_view: Option<bool>,
    /// Pool view exception.
    #[serde(rename="poolViewException")]
    
    pub pool_view_exception: Option<ViewsFromUnitPoolViewExceptionEnum>,
    /// Valley view. A guestroom that features a window through which guests can see over a valley.
    #[serde(rename="valleyView")]
    
    pub valley_view: Option<bool>,
    /// Valley view exception.
    #[serde(rename="valleyViewException")]
    
    pub valley_view_exception: Option<ViewsFromUnitValleyViewExceptionEnum>,
}

impl client::Part for ViewsFromUnit {}


/// Waste reduction practices implemented at the hotel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WasteReduction {
    /// Compostable food containers and cutlery. 100% of food service containers and to-go cutlery are compostable, and reusable utensils are offered wherever possible. Compostable materials are capable of undergoing biological decomposition in a compost site, such that material is not visually distinguishable and breaks down into carbon dioxide, water, inorganic compounds, and biomass.
    #[serde(rename="compostableFoodContainersAndCutlery")]
    
    pub compostable_food_containers_and_cutlery: Option<bool>,
    /// Compostable food containers and cutlery exception.
    #[serde(rename="compostableFoodContainersAndCutleryException")]
    
    pub compostable_food_containers_and_cutlery_exception: Option<WasteReductionCompostableFoodContainersAndCutleryExceptionEnum>,
    /// Composts excess food. The property has a program and/or policy for diverting waste from landfill by composting food and yard waste, either through compost collection and off-site processing or on-site compost processing.
    #[serde(rename="compostsExcessFood")]
    
    pub composts_excess_food: Option<bool>,
    /// Composts excess food exception.
    #[serde(rename="compostsExcessFoodException")]
    
    pub composts_excess_food_exception: Option<WasteReductionCompostsExcessFoodExceptionEnum>,
    /// Donates excess food. The property has a program and/or policy for diverting waste from landfill that may include efforts to donate for human consumption or divert food for animal feed.
    #[serde(rename="donatesExcessFood")]
    
    pub donates_excess_food: Option<bool>,
    /// Donates excess food exception.
    #[serde(rename="donatesExcessFoodException")]
    
    pub donates_excess_food_exception: Option<WasteReductionDonatesExcessFoodExceptionEnum>,
    /// Food waste reduction program. The property has established a food waste reduction and donation program, aiming to reduce food waste by half. These programs typically use tools such as the Hotel Kitchen Toolkit and others to track waste and measure progress.
    #[serde(rename="foodWasteReductionProgram")]
    
    pub food_waste_reduction_program: Option<bool>,
    /// Food waste reduction program exception.
    #[serde(rename="foodWasteReductionProgramException")]
    
    pub food_waste_reduction_program_exception: Option<WasteReductionFoodWasteReductionProgramExceptionEnum>,
    /// No single use plastic straws. The property bans single-use plastic straws.
    #[serde(rename="noSingleUsePlasticStraws")]
    
    pub no_single_use_plastic_straws: Option<bool>,
    /// No single use plastic straws exception.
    #[serde(rename="noSingleUsePlasticStrawsException")]
    
    pub no_single_use_plastic_straws_exception: Option<WasteReductionNoSingleUsePlasticStrawsExceptionEnum>,
    /// No single use plastic water bottles. The property bans single-use plastic water bottles.
    #[serde(rename="noSingleUsePlasticWaterBottles")]
    
    pub no_single_use_plastic_water_bottles: Option<bool>,
    /// No single use plastic water bottles exception.
    #[serde(rename="noSingleUsePlasticWaterBottlesException")]
    
    pub no_single_use_plastic_water_bottles_exception: Option<WasteReductionNoSingleUsePlasticWaterBottlesExceptionEnum>,
    /// No styrofoam food containers. The property eliminates the use of Styrofoam in disposable food service items.
    #[serde(rename="noStyrofoamFoodContainers")]
    
    pub no_styrofoam_food_containers: Option<bool>,
    /// No styrofoam food containers exception.
    #[serde(rename="noStyrofoamFoodContainersException")]
    
    pub no_styrofoam_food_containers_exception: Option<WasteReductionNoStyrofoamFoodContainersExceptionEnum>,
    /// Recycling program. The property has a recycling program, aligned with LEED waste requirements, and a policy outlining efforts to send less than 50% of waste to landfill. The recycling program includes storage locations for recyclable materials, including mixed paper, corrugated cardboard, glass, plastics, and metals.
    #[serde(rename="recyclingProgram")]
    
    pub recycling_program: Option<bool>,
    /// Recycling program exception.
    #[serde(rename="recyclingProgramException")]
    
    pub recycling_program_exception: Option<WasteReductionRecyclingProgramExceptionEnum>,
    /// Refillable toiletry containers. The property has replaced miniature individual containers with refillable amenity dispensers for shampoo, conditioner, soap, and lotion.
    #[serde(rename="refillableToiletryContainers")]
    
    pub refillable_toiletry_containers: Option<bool>,
    /// Refillable toiletry containers exception.
    #[serde(rename="refillableToiletryContainersException")]
    
    pub refillable_toiletry_containers_exception: Option<WasteReductionRefillableToiletryContainersExceptionEnum>,
    /// Safely disposes batteries. The property safely stores and disposes batteries.
    #[serde(rename="safelyDisposesBatteries")]
    
    pub safely_disposes_batteries: Option<bool>,
    /// Safely disposes batteries exception.
    #[serde(rename="safelyDisposesBatteriesException")]
    
    pub safely_disposes_batteries_exception: Option<WasteReductionSafelyDisposesBatteriesExceptionEnum>,
    /// Safely disposes electronics. The property has a reputable recycling program that keeps hazardous electronic parts and chemical compounds out of landfills, dumps and other unauthorized abandonment sites, and recycles/reuses applicable materials. (e.g. certified electronics recyclers).
    #[serde(rename="safelyDisposesElectronics")]
    
    pub safely_disposes_electronics: Option<bool>,
    /// Safely disposes electronics exception.
    #[serde(rename="safelyDisposesElectronicsException")]
    
    pub safely_disposes_electronics_exception: Option<WasteReductionSafelyDisposesElectronicsExceptionEnum>,
    /// Safely disposes lightbulbs. The property safely stores and disposes lightbulbs.
    #[serde(rename="safelyDisposesLightbulbs")]
    
    pub safely_disposes_lightbulbs: Option<bool>,
    /// Safely disposes lightbulbs exception.
    #[serde(rename="safelyDisposesLightbulbsException")]
    
    pub safely_disposes_lightbulbs_exception: Option<WasteReductionSafelyDisposesLightbulbsExceptionEnum>,
    /// Safely handles hazardous substances. The property has a hazardous waste management program aligned wit GreenSeal and LEED requirements, and meets all regulatory requirements for hazardous waste disposal and recycling. Hazardous means substances that are classified as "hazardous" by an authoritative body (such as OSHA or DOT), are labeled with signal words such as "Danger," "Caution," "Warning," or are flammable, corrosive, or ignitable. Requirements include: - The property shall maintain records of the efforts it has made to replace the hazardous substances it uses with less hazardous alternatives. - An inventory of the hazardous materials stored on-site. - Products intended for cleaning, dishwashing, laundry, and pool maintenance shall be stored in clearly labeled containers. These containers shall be checked regularly for leaks, and replaced a necessary. - Spill containment devices shall be installed to collect spills, drips, or leaching of chemicals.
    #[serde(rename="safelyHandlesHazardousSubstances")]
    
    pub safely_handles_hazardous_substances: Option<bool>,
    /// Safely handles hazardous substances exception.
    #[serde(rename="safelyHandlesHazardousSubstancesException")]
    
    pub safely_handles_hazardous_substances_exception: Option<WasteReductionSafelyHandlesHazardousSubstancesExceptionEnum>,
    /// Soap donation program. The property participates in a soap donation program such as Clean the World or something similar.
    #[serde(rename="soapDonationProgram")]
    
    pub soap_donation_program: Option<bool>,
    /// Soap donation program exception.
    #[serde(rename="soapDonationProgramException")]
    
    pub soap_donation_program_exception: Option<WasteReductionSoapDonationProgramExceptionEnum>,
    /// Toiletry donation program. The property participates in a toiletry donation program such as Clean the World or something similar.
    #[serde(rename="toiletryDonationProgram")]
    
    pub toiletry_donation_program: Option<bool>,
    /// Toiletry donation program exception.
    #[serde(rename="toiletryDonationProgramException")]
    
    pub toiletry_donation_program_exception: Option<WasteReductionToiletryDonationProgramExceptionEnum>,
    /// Water bottle filling stations. The property offers water stations throughout the building for guest use.
    #[serde(rename="waterBottleFillingStations")]
    
    pub water_bottle_filling_stations: Option<bool>,
    /// Water bottle filling stations exception.
    #[serde(rename="waterBottleFillingStationsException")]
    
    pub water_bottle_filling_stations_exception: Option<WasteReductionWaterBottleFillingStationsExceptionEnum>,
}

impl client::Part for WasteReduction {}


/// Water conservation practices implemented at the hotel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterConservation {
    /// Independent organization audits water use. The property conducts a water conservation audit every 5 years, the results of which are either verified by a third-party and/or published in external communications. A water conservation audit is a detailed assessment of the facility, providing recommendations to existing operations and procedures to improve water efficiency, available incentives or rebates, and opportunities for improvements through renovations or upgrades. Examples of organizations who conduct credible third party audits include: Engie Impact, and local utility providers (they often provide energy and water audits).
    #[serde(rename="independentOrganizationAuditsWaterUse")]
    
    pub independent_organization_audits_water_use: Option<bool>,
    /// Independent organization audits water use exception.
    #[serde(rename="independentOrganizationAuditsWaterUseException")]
    
    pub independent_organization_audits_water_use_exception: Option<WaterConservationIndependentOrganizationAuditsWaterUseExceptionEnum>,
    /// Linen reuse program. The property offers a linen reuse program.
    #[serde(rename="linenReuseProgram")]
    
    pub linen_reuse_program: Option<bool>,
    /// Linen reuse program exception.
    #[serde(rename="linenReuseProgramException")]
    
    pub linen_reuse_program_exception: Option<WaterConservationLinenReuseProgramExceptionEnum>,
    /// Towel reuse program. The property offers a towel reuse program.
    #[serde(rename="towelReuseProgram")]
    
    pub towel_reuse_program: Option<bool>,
    /// Towel reuse program exception.
    #[serde(rename="towelReuseProgramException")]
    
    pub towel_reuse_program_exception: Option<WaterConservationTowelReuseProgramExceptionEnum>,
    /// Water saving showers. All of the property's guest rooms have shower heads that use no more than 2.0 gallons per minute (gpm).
    #[serde(rename="waterSavingShowers")]
    
    pub water_saving_showers: Option<bool>,
    /// Water saving showers exception.
    #[serde(rename="waterSavingShowersException")]
    
    pub water_saving_showers_exception: Option<WaterConservationWaterSavingShowersExceptionEnum>,
    /// Water saving sinks. All of the property's guest rooms have bathroom faucets that use a maximum of 1.5 gallons per minute (gpm), public restroom faucets do not exceed 0.5 gpm, and kitchen faucets (excluding faucets used exclusively for filling operations) do not exceed 2.2 gpm.
    #[serde(rename="waterSavingSinks")]
    
    pub water_saving_sinks: Option<bool>,
    /// Water saving sinks exception.
    #[serde(rename="waterSavingSinksException")]
    
    pub water_saving_sinks_exception: Option<WaterConservationWaterSavingSinksExceptionEnum>,
    /// Water saving toilets. All of the property's toilets use 1.6 gallons per flush, or less.
    #[serde(rename="waterSavingToilets")]
    
    pub water_saving_toilets: Option<bool>,
    /// Water saving toilets exception.
    #[serde(rename="waterSavingToiletsException")]
    
    pub water_saving_toilets_exception: Option<WaterConservationWaterSavingToiletsExceptionEnum>,
}

impl client::Part for WaterConservation {}


/// Guest facilities at the property to promote or maintain health, beauty, and fitness.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Wellness {
    /// Doctor on call. The hotel has a contract with a medical professional who provides services to hotel guests should they fall ill during their stay. The doctor may or may not have an on-site office or be at the hotel at all times.
    #[serde(rename="doctorOnCall")]
    
    pub doctor_on_call: Option<bool>,
    /// Doctor on call exception.
    #[serde(rename="doctorOnCallException")]
    
    pub doctor_on_call_exception: Option<WellnesDoctorOnCallExceptionEnum>,
    /// Elliptical machine. An electric, stationary fitness machine with pedals that simulates climbing, walking or running and provides a user-controlled range of speeds and tensions. May not have arm-controlled levers to work out the upper body as well. Commonly found in a gym, fitness room, health center, or health club.
    #[serde(rename="ellipticalMachine")]
    
    pub elliptical_machine: Option<bool>,
    /// Elliptical machine exception.
    #[serde(rename="ellipticalMachineException")]
    
    pub elliptical_machine_exception: Option<WellnesEllipticalMachineExceptionEnum>,
    /// Fitness center. A room or building at the hotel containing equipment to promote physical activity, such as treadmills, elliptical machines, stationary bikes, weight machines, free weights, and/or stretching mats. Use of the fitness center can be free or for a fee. May or may not be staffed. May or may not offer instructor-led classes in various styles of physical conditioning. May or may not be open 24/7. May or may not include locker rooms and showers. Also known as health club, gym, fitness room, health center.
    #[serde(rename="fitnessCenter")]
    
    pub fitness_center: Option<bool>,
    /// Fitness center exception.
    #[serde(rename="fitnessCenterException")]
    
    pub fitness_center_exception: Option<WellnesFitnessCenterExceptionEnum>,
    /// Free fitness center. Guests may use the fitness center for free.
    #[serde(rename="freeFitnessCenter")]
    
    pub free_fitness_center: Option<bool>,
    /// Free fitness center exception.
    #[serde(rename="freeFitnessCenterException")]
    
    pub free_fitness_center_exception: Option<WellnesFreeFitnessCenterExceptionEnum>,
    /// Free weights. Individual handheld fitness equipment of varied weights used for upper body strength training or bodybuilding. Also known as barbells, dumbbells, or kettlebells. Often stored on a rack with the weights arranged from light to heavy. Commonly found in a gym, fitness room, health center, or health club.
    #[serde(rename="freeWeights")]
    
    pub free_weights: Option<bool>,
    /// Free weights exception.
    #[serde(rename="freeWeightsException")]
    
    pub free_weights_exception: Option<WellnesFreeWeightsExceptionEnum>,
    /// Massage. A service provided by a trained massage therapist involving the physical manipulation of a guest's muscles in order to achieve relaxation or pain relief.
    
    pub massage: Option<bool>,
    /// Massage exception.
    #[serde(rename="massageException")]
    
    pub massage_exception: Option<WellnesMassageExceptionEnum>,
    /// Salon. A room at the hotel where professionals provide hair styling services such as shampooing, blow drying, hair dos, hair cutting and hair coloring. Also known as hairdresser or beauty salon.
    
    pub salon: Option<bool>,
    /// Salon exception.
    #[serde(rename="salonException")]
    
    pub salon_exception: Option<WellnesSalonExceptionEnum>,
    /// Sauna. A wood-paneled room heated to a high temperature where guests sit on built-in wood benches for the purpose of perspiring and relaxing their muscles. Can be dry or slightly wet heat. Not a steam room.
    
    pub sauna: Option<bool>,
    /// Sauna exception.
    #[serde(rename="saunaException")]
    
    pub sauna_exception: Option<WellnesSaunaExceptionEnum>,
    /// Spa. A designated area, room or building at the hotel offering health and beauty treatment through such means as steam baths, exercise equipment, and massage. May also offer facials, nail care, and hair care. Services are usually available by appointment and for an additional fee. Does not apply if hotel only offers a steam room; must offer other beauty and/or health treatments as well.
    
    pub spa: Option<bool>,
    /// Spa exception.
    #[serde(rename="spaException")]
    
    pub spa_exception: Option<WellnesSpaExceptionEnum>,
    /// Treadmill. An electric stationary fitness machine that simulates a moving path to promote walking or running within a range of user-controlled speeds and inclines. Also known as running machine. Commonly found in a gym, fitness room, health center, or health club.
    
    pub treadmill: Option<bool>,
    /// Treadmill exception.
    #[serde(rename="treadmillException")]
    
    pub treadmill_exception: Option<WellnesTreadmillExceptionEnum>,
    /// Weight machine. Non-electronic fitness equipment designed for the user to target the exertion of different muscles. Usually incorporates a padded seat, a stack of flat weights and various bars and pulleys. May be designed for toning a specific part of the body or may involve different user-controlled settings, hardware and pulleys so as to provide an overall workout in one machine. Commonly found in a gym, fitness center, fitness room, or health club.
    #[serde(rename="weightMachine")]
    
    pub weight_machine: Option<bool>,
    /// Weight machine exception.
    #[serde(rename="weightMachineException")]
    
    pub weight_machine_exception: Option<WellnesWeightMachineExceptionEnum>,
}

impl client::Part for Wellness {}


