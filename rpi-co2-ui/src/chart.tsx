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
import { useEffect, useState } from "react";
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

async function fetchMeasurements(serverAdress: String, setValues: (values: CO2DataPoint[]) => void): Promise<void> {
  return await fetch(`${serverAdress}/data`, {})
    .then((res) => {
      if (!res.ok || res.status >= 300) {
        alert("Error fetching data");
      }
      return res.json();
    })
    .then((data) => {
      console.log(data, "from", serverAdress);
      const results: CO2DataPoint[] = [];
      data["timestamps"].forEach((ts: string, idx: number) => {
        results.push({ timestamp: new Date(ts), value: data["co2values"][idx] });
      });
      setValues(results);
    });
}

const HealthCheck = ({ serverAdress }: { serverAdress: String }) => {
  const [health, setHealth] = useState<boolean>(false);

  useEffect(() => {
    const updateHealth = async () => {
      try {
        const response = await fetch(`${serverAdress}/health`);
        const data = await response.text();
        setHealth(data == "ok");
      } catch (error) {
        console.error("Failed to fetch health status", error);
        setHealth(false);
      }
    };

    updateHealth(); // Initial check
    const interval = setInterval(updateHealth, 1000); // Update every second
    return () => clearInterval(interval); // Cleanup on component unmount
  }, [serverAdress]);

  return (
    <h2>{health ? "Healthy" : "Unhealthy"}</h2>
    // <div style={{ padding: "10px" }}>
    //   <div className="flex items-center space-x-2">
    //     <div className={`h-2 w-2 rounded-full ${health ? "bg-green-500" : "bg-red-500"}`}></div>
    //     <span className="text-sm font-medium">{health ? "Healthy" : "Unhealthy"}</span>
    //   </div>
    // </div>
  );
};

export function Chart({ serverAdress }: { serverAdress: string }) {
  const [measurements, setMeasurements] = useState<CO2DataPoint[]>(dummyMeasurements);
  const [lastN, setLastN] = useState<number>(100);
  const [refreshTimeMS, setRefreshTimeMS] = useState<number>(1000);

  let sortedData = measurements.sort((a, b) => (a.timestamp > b.timestamp ? 1 : -1));
  sortedData = sortedData.slice(-lastN);
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

  useEffect(() => {
    const interval = setInterval(() => {
      fetchMeasurements(serverAdress, setMeasurements);
    }, refreshTimeMS);

    return () => clearInterval(interval); // Cleanup interval on component unmount
  }, [serverAdress, refreshTimeMS]);

  return (
    <div style={{ width: "80vw", height: "50vh" }}>
      <HealthCheck serverAdress={serverAdress} />
      <Line options={options} data={data} />
      <div>
        <label>Last N</label>
        <input type="number" value={lastN} onChange={(e) => setLastN(parseInt(e.target.value))} />
      </div>
      <div>
        <label>Refresh Time (ms)</label>
        <input type="number" value={refreshTimeMS} onChange={(e) => setRefreshTimeMS(parseInt(e.target.value))} />
      </div>
    </div>
  );
}
