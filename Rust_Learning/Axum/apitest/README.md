## The first section contains the pure JSON data, and the second section is a detailed explanation formatted as a README.md file.

---
API: https://www.gdacs.org/gdacsapi/api/events/geteventlist/EVENTS4APP
---

## Pure JSON Data

```json
{
  "type": "Feature",
  "bbox": [
    -62.2361,
    10.9061,
    -62.2361,
    10.9061
  ],
  "geometry": {
    "type": "Point",
    "coordinates": [
      -62.2361,
      10.9061
    ]
  },
  "properties": {
    "eventtype": "EQ",
    "eventid": 1468716,
    "episodeid": 1623105,
    "eventname": "",
    "glide": "",
    "name": "Earthquake in Venezuela",
    "description": "Earthquake in Venezuela",
    "htmldescription": "Green M 4.5 Earthquake in Venezuela at: 17 Feb 2025 05:27:50.",
    "icon": "https://www.gdacs.org/images/gdacs_icons/maps/Green/EQ.png",
    "iconoverall": "https://www.gdacs.org/images/gdacs_icons/maps/Green/EQ.png",
    "url": {
      "geometry": "https://www.gdacs.org/gdacsapi/api/polygons/getgeometry?eventtype=EQ&eventid=1468716&episodeid=1623105",
      "report": "https://www.gdacs.org/report.aspx?eventid=1468716&episodeid=1623105&eventtype=EQ",
      "details": "https://www.gdacs.org/gdacsapi/api/events/geteventdata?eventtype=EQ&eventid=1468716"
    },
    "alertlevel": "Green",
    "alertscore": 1,
    "episodealertlevel": "Green",
    "episodealertscore": 0,
    "istemporary": "false",
    "iscurrent": "true",
    "country": "Venezuela",
    "fromdate": "2025-02-17T05:27:50",
    "todate": "2025-02-17T05:27:50",
    "datemodified": "2025-02-17T06:08:14",
    "iso3": "VEN",
    "source": "NEIC",
    "sourceid": "",
    "polygonlabel": "Centroid",
    "Class": "Point_Centroid",
    "affectedcountries": [
      {
        "iso2": "VE",
        "iso3": "VEN",
        "countryname": "Venezuela"
      }
    ],
    "severitydata": {
      "severity": 4.5,
      "severitytext": "Magnitude 4.5M, Depth:100.785km",
      "severityunit": "M"
    }
  }
}
```

---

## README.md Explanation

# GDACS Event Data Explanation

This document explains each part of the GDACS event data provided in the JSON above. It is structured to help you understand what each field represents.

## 1. Top-Level Structure

- **`type`: "Feature"**  
  Indicates that the JSON object follows the GeoJSON standard and represents a spatial feature.

- **`bbox`: [ ... ]**  
  A bounding box that defines the spatial extent of the feature.  
  - Format: `[min_longitude, min_latitude, max_longitude, max_latitude]`.  
  - For a point, both the minimum and maximum coordinates are identical.

## 2. Geometry Object

- **`geometry`**  
  Contains the spatial (geographic) information of the event.
  - **`type`: "Point"**  
    Specifies that the geometry is a point.
  - **`coordinates`: [ -62.2361, 10.9061 ]**  
    - The **first number** is the **longitude** (`-62.2361`).
    - The **second number** is the **latitude** (`10.9061`).  
    *Note: In GeoJSON, the coordinate order is always [longitude, latitude].*

## 3. Properties Object

This section contains detailed metadata about the event:

- **`eventtype`:**  

--------
- TC: Tropical Cyclones
- EQ: Earthquakes
- FL: Floods
- VO: Volcanoes
- WF: Wild Fires
- DR: Droughts
--------


- **`eventid`: 1468716**  
  A unique identifier for this specific event.

- **`episodeid`: 1623105**  
  Identifies the episode or specific occurrence of the event, useful if the event has multiple updates or phases.

- **`eventname`: ""**  
  A field intended for the event's name. It is empty in this example.

- **`glide`: ""**  
  Reserved for an additional code or identifier related to the event; empty here.

- **`name`: "Earthquake in Venezuela"**  
  A clear, descriptive title of the event.

- **`description`: "Earthquake in Venezuela"**  
  A brief summary of what the event is.

- **`htmldescription`: "Green M 4.5 Earthquake in Venezuela at: 17 Feb 2025 05:27:50."**  
  An HTML-formatted version of the description, which can be used for displaying rich text on web pages.

- **`icon` and `iconoverall`:**  
  URLs to icon images representing the event.  
  - These icons visually indicate the type of event (here, an earthquake).

- **`url`: { ... }**  
  An object that holds related URLs for further information:
  - **`geometry`:** Link to the event's geometry or polygon data.
  - **`report`:** Link to a detailed report about the event.
  - **`details`:** Link to more comprehensive event data via the API.

- **`alertlevel`:**  
-------------
Based on the definitions provided and GDACS conventions, the API categorizes events using four distinct alert levels, each corresponding to an expected need for international assistance:

- **White Alerts**  
  These indicate a minor event where the likelihood of needing international assistance is very low.

- **Green Alerts**  
  These represent moderate events, suggesting that while the situation may require local attention, international assistance is not likely to be necessary.

- **Orange Alerts**  
  These are used for potential local disasters where international assistance might be required if the situation escalates.

- **Red Alerts**  
  These denote potentially severe disasters where international assistance is expected to be required due to the high level of impact.
-------------


- **`alertscore`: 1**  
  A numerical representation of the alert level.

- **`episodealertlevel`: "Green"** and **`episodealertscore`: 0**  
  Specific alert information for this episode of the event.

- **`istemporary`: "false"**  
  Indicates whether the event is temporary. Here, it is marked as false.

- **`iscurrent`: "true"**  
  Indicates if the event is currently active.

- **`country`: "Venezuela"**  
  The country where the event is occurring.

- **`fromdate` and `todate`: "2025-02-17T05:27:50"**  
  The starting and ending times of the event. For instantaneous events like earthquakes, these timestamps are the same.

- **`datemodified`: "2025-02-17T06:08:14"**  
  The timestamp showing when the event data was last updated.

- **`iso3`: "VEN"**  
  The ISO 3166-1 alpha-3 country code for Venezuela.

- **`source`: "NEIC"**  
  The data source, with NEIC standing for the National Earthquake Information Center.

- **`sourceid`: ""**  
  An additional source identifier; it is empty in this example.

- **`polygonlabel`: "Centroid"** and **`Class`: "Point_Centroid"**  
  Indicate that the provided coordinate represents the centroid (geometric center) of the event’s affected area.

- **`affectedcountries`: [ ... ]**  
  An array of objects listing the countries affected by the event.
  - Each object contains:
    - **`iso2`: "VE"** (two-letter country code)
    - **`iso3`: "VEN"** (three-letter country code)
    - **`countryname`: "Venezuela"** (full country name)

- **`severitydata`: { ... }**  
  Provides details on the event’s severity:
  - **`severity`: 4.5**  
    Numerical value representing the event's magnitude (for earthquakes).
  - **`severitytext`: "Magnitude 4.5M, Depth:100.785km"**  
    Descriptive text including both magnitude and depth information.
  - **`severityunit`: "M"**  
    The unit of measurement for the severity (M indicates Magnitude).
