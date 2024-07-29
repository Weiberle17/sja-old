import React, { useState, useEffect } from "react";
import { Tabs, Tab, Box, Typography, CircularProgress } from "@mui/material";
import CustomTable from "./components/CustomTable";

const tabData = [
  { label: "Angebote", endpoint: "http://localhost:3001/api/db/angebote" },
  {
    label: "Organisationen",
    endpoint: "http://localhost:3001/api/db/organisationen",
  },
  {
    label: "Ansprechpartner",
    endpoint: "http://localhost:3001/api/db/ansprechpartner",
  },
];

const TabbedList = () => {
  const [selectedTab, setSelectedTab] = useState(0);
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Fetch data from the server whenever the selected tab changes
    const fetchData = async () => {
      setLoading(true);
      setError(null);
      try {
        const response = await fetch(tabData[selectedTab].endpoint);
        if (!response.ok) {
          throw new Error("Network response was not ok");
        }
        const result = await response.json();
        setData(result);
      } catch (error) {
        setError(error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [selectedTab]);

  const handleChange = (_event, newValue) => {
    setSelectedTab(newValue);
  };

  return (
    <Box sx={{ width: "100%" }}>
      <Tabs value={selectedTab} onChange={handleChange} centered>
        {tabData.map((tab, index) => (
          <Tab label={tab.label} key={index} />
        ))}
      </Tabs>

      <Box sx={{ p: 2 }}>
        {loading && <CircularProgress />}
        {error && (
          <Typography color="error">
            Failed to load data: {error.message}
          </Typography>
        )}
        {!loading && !error && (
          <CustomTable data={data} tabIndex={selectedTab} />
        )}
      </Box>
    </Box>
  );
};

export default TabbedList;
