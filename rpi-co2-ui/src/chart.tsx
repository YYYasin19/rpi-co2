import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { useState } from "react";
import { Line } from "react-chartjs-2";

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend);

export const options = {
  responsive: true,
  maintainAspectRatio: false, // Ensure the chart is not bound by its aspect ratio
  plugins: {
    legend: {
      position: "top" as const,
    },
    title: {
      display: false,
      text: "Chart.js Line Chart",
    },
  },
};

type CO2DataPoint = {
  timestamp: Date;
  value: number;
};
const dummyMeasurements: CO2DataPoint[] = [
  {
    timestamp: new Date("01.01.2023"),
    value: 1000,
  },
  {
    timestamp: new Date("01.02.2023"),
    value: 1200,
  },
  {
    timestamp: new Date("01.03.2023"),
    value: 1300,
  },
  {
    timestamp: new Date("01.04.2023"),
    value: 1100,
  },
];

async function fetchMeasurements(setValues: (values: CO2DataPoint[]) => void): Promise<void> {
  return await fetch("http://localhost:3000/data", {})
    .then((res) => {
      if (!res.ok || res.status >= 300) {
        alert("Error fetching data");
      }
      return res.json();
    })
    .then((data) => {
      console.log(data);
      const results: CO2DataPoint[] = [];
      data["timestamps"].forEach((ts: string, idx: number) => {
        results.push({ timestamp: new Date(ts), value: data["co2values"][idx] });
      });
      setValues(results);
    });
}

export function Chart() {
  const [measurements, setMeasurements] = useState<CO2DataPoint[]>(dummyMeasurements);

  const sortedData = measurements.sort((a, b) => (a.timestamp > b.timestamp ? 1 : -1));
  const data = {
    labels: sortedData.map((d) => d.timestamp.toLocaleTimeString()),
    datasets: [
      {
        label: "CO2 Measurements",
        data: sortedData.map((d) => d.value),
        fill: false,
        borderColor: "rgb(255, 99, 132)",
        backgroundColor: "rgba(255, 99, 132, 0.5)",
      },
    ],
  };
  return (
    <div style={{ width: "80vw", height: "50vh" }}>
      {" "}
      {/* Adjusted to use viewport width and height for full screen */}
      <button onClick={() => fetchMeasurements(setMeasurements)}>Reload</button>
      <Line options={options} data={data} />
    </div>
  );
}
