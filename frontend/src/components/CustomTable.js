import React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Typography,
} from "@mui/material";

const CustomTable = ({ data, tabIndex: tabName }) => {
  if (!data.length) {
    return <Typography>No data available</Typography>;
  }

  // Define column headers based on the selected tab
  let columns = [];
  if (tabName === 0) {
    columns = ["Name", "Beschreibung", "Organisation"];
  } else if (tabName === 1) {
    columns = ["Name", "Ansprechpartner"];
  } else if (tabName === 2) {
    columns = ["Nachname", "Vorname", "Telefon", "Email"];
  }

  // Render the table
  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            {columns.map((col) => (
              <TableCell key={col}>{col}</TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          {data.map((item, index) => (
            <TableRow key={index}>
              {columns.map((col) => (
                <TableCell key={col}>
                  {col === "Name" && tabName === 0
                    ? item.angebot_name
                    : col === "Email" && tabName === 0
                      ? item.email
                      : col === "Age" && tabName === 0
                        ? item.age
                        : col === "Name" && tabName === 1
                          ? item.organisation_name
                          : col === "Ansprechpartner" && tabName === 1
                            ? item.Organisation_ansprechpartner
                            : col === "Date" && tabName === 1
                              ? item.date
                              : col === "Item" && tabName === 2
                                ? item.item
                                : col === "Details" && tabName === 2
                                  ? item.details
                                  : "N/A"}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default CustomTable;
