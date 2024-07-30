"use client";
import Filter from "@/components/Filter";
import CustomTable from "@/components/CustomTable";
import React, { useEffect, useState } from "react";
import { Tabs, Tab, Box, Typography, CircularProgress } from "@mui/material";
import { Angebot } from "@/types";
import dotenv from "dotenv";

// BUG: This always defaults to the or - no idea why <2024-07-30>
dotenv.config();
const apiURL = process.env.BACKEND_API || "http://localhost:3001/api";

const tabData = [
  { label: "Angebote", endpoint: `${apiURL}/db/angebote` },
  {
    label: "Veraltet",
    endpoint: `${apiURL}/db/outdated`,
  },
  // {
  //   label: "Ansprechpartner",
  //   endpoint: `${apiURL}/db/ansprechpartner`,
  // },
];

const App: React.FC = () => {
  const [selectedTab, setSelectedTab] = useState(0);
  const [selectedCategories, setSelectedCategories] = useState<string[]>([]);
  const [data, setData] = useState<Angebot[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<any>(null);

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
        console.log(result);
        setData(result);
      } catch (error: any) {
        setError(error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [selectedTab]);

  const handleChange = (_event: any, newValue: number) => {
    setSelectedTab(newValue);
  };

  const handleFilterChange = (category: string) => {
    setSelectedCategories((prev) =>
      prev.includes(category)
        ? prev.filter((c) => c !== category)
        : [...prev, category],
    );
  };

  const filteredEntries = selectedCategories.length
    ? data.filter((d) => selectedCategories.includes(d.angebot.kategorie))
    : data;

  return (
    <Box sx={{ width: "100%" }}>
      <Tabs value={selectedTab} onChange={handleChange} centered>
        {tabData.map((tab, index) => (
          <Tab label={tab.label} key={index} />
        ))}
      </Tabs>

      <Box sx={{ p: 2 }}>
        <Filter onFilterChange={handleFilterChange} />
      </Box>

      <Box sx={{ p: 2 }}>
        {loading && <CircularProgress />}
        {error && (
          <Typography color="error">
            Failed to load data: {error.message}
          </Typography>
        )}
        {!loading && !error && (
          <CustomTable data={filteredEntries} />
        )}
      </Box>
    </Box>
  );
};

export default App;
