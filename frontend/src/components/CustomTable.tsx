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
import { Angebot } from "@/types";

const CustomTable = ({ data }: { data: Angebot[] }) => {
  if (!data.length) {
    return <Typography>No data available</Typography>;
  }

  // Define column headers based on the selected tab
  let columns = ["Name", "Link(s)", "Beschreibung", "Organisation", "Adresse(n)", "Stadtteil(e)", "Telefonnummer(n)", "Email(s)", "Sonstiges"];

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
          {data.map((d, index) => (
            <TableRow key={index}>
              {columns.map((col) => (
                <TableCell key={col}>
                  {col === "Name"
                    ? d.angebot.angebot_name
                    : col === "Link(s)"
                      ? d.links.map(link => link.link).join(', ')
                      : col === "Beschreibung"
                        ? d.angebot.beschreibung
                        : col === "Organisation"
                          ? d.organisation.organisation.organisation_name
                          : col === "Adresse(n)"
                            ? d.adressen.map(addr => (addr.strasse, addr.hausnr, addr.plz)).join(', ')
                            : col === "Stadtteil(e)"
                              ? d.adressen.map(addr => addr.stadtteil).join(', ')
                              : col === "Telefonnummer(n)"
                                ? d.apartner[0].vor_name
                                : col === "Email(s)"
                                  ? d.apartner[0].nach_name
                                  : col === "Sonstiges"
                                    ? d.sonstiges.map(sonstiges => sonstiges.text).join(', ')
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
