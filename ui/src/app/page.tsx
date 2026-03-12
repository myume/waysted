"use client";

import { useState, useEffect, useMemo, useRef } from "react";
import {
  Calendar as CalendarIcon,
  Clock,
  BarChart3,
  List,
  ChevronLeft,
  ChevronRight,
  Monitor,
  Search,
  Filter,
  X,
  ChevronDown,
} from "lucide-react";
import { format, addDays, subDays, isSameDay } from "date-fns";
import { DayPicker } from "react-day-picker";
import "react-day-picker/style.css";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Rectangle,
} from "recharts";
import type { TooltipValueType, BarShapeProps } from "recharts";
import { cn, formatDuration, formatShortDuration } from "@/lib/utils";
import type { AppScreentime, ScreenTimeInstance } from "@/lib/cli";

export default function WaystedUI() {
  const [selectedDate, setSelectedDate] = useState<Date>(new Date());
  const [showCalendar, setShowCalendar] = useState(false);
  const calendarRef = useRef<HTMLDivElement>(null);

  const [view, setView] = useState<"dashboard" | "logs" | "schedule">(
    "dashboard",
  );
  const [screentime, setScreentime] = useState<AppScreentime[]>([]);
  const [logs, setLogs] = useState<ScreenTimeInstance[]>([]);
  const [loading, setLoading] = useState(true);

  // Log Filters
  const [searchTerm, setSearchTerm] = useState("");
  const [appFilter, setAppFilter] = useState("All Apps");
  const [startTimeFilter, setStartTimeFilter] = useState("00:00");
  const [endTimeFilter, setEndTimeFilter] = useState("23:59");

  const dateStr = format(selectedDate, "yyyy-MM-dd");

  // Close calendar on outside click
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        calendarRef.current &&
        !calendarRef.current.contains(event.target as Node)
      ) {
        setShowCalendar(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  useEffect(() => {
    async function fetchData() {
      setLoading(true);
      try {
        const [resScreentime, resLogs] = await Promise.all([
          fetch(`/api/data?date=${dateStr}&type=screentime`).then((r) =>
            r.json(),
          ),
          fetch(`/api/data?date=${dateStr}&type=logs`).then((r) => r.json()),
        ]);

        setScreentime(Array.isArray(resScreentime) ? resScreentime : []);
        setLogs(Array.isArray(resLogs) ? resLogs : []);
      } catch (error) {
        console.error("Failed to fetch data:", error);
      } finally {
        setLoading(false);
      }
    }
    fetchData();
  }, [dateStr]);

  const appNames = useMemo(() => {
    const names = Array.from(new Set(logs.map((log) => log.app_name)));
    return ["All Apps", ...names.sort()];
  }, [logs]);

  const filteredLogs = useMemo(() => {
    return logs.filter((log) => {
      const logDate = new Date(log.start_timestamp);
      const logTimeStr = format(logDate, "HH:mm");

      const matchesSearch =
        !searchTerm ||
        log.app_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        log.title.toLowerCase().includes(searchTerm.toLowerCase());

      const matchesApp = appFilter === "All Apps" || log.app_name === appFilter;

      const matchesTime =
        logTimeStr >= startTimeFilter && logTimeStr <= endTimeFilter;

      return matchesSearch && matchesApp && matchesTime;
    });
  }, [logs, searchTerm, appFilter, startTimeFilter, endTimeFilter]);

  const totalDuration = useMemo(() => {
    return screentime.reduce((acc, curr) => acc + curr.duration, 0);
  }, [screentime]);

  // Schedule View Calculations
  const hourlyData = useMemo(() => {
    const hours = Array.from({ length: 24 }, (_, i) => ({
      hour: i,
      label: `${i}:00`,
      duration: 0,
      apps: {} as Record<string, number>,
    }));

    logs.forEach((log) => {
      const start = new Date(log.start_timestamp);
      const hour = start.getHours();
      hours[hour].duration += log.duration;
      hours[hour].apps[log.app_name] =
        (hours[hour].apps[log.app_name] || 0) + log.duration;
    });

    return hours;
  }, [logs]);

  const COLORS = [
    "#3b82f6",
    "#10b981",
    "#f59e0b",
    "#ef4444",
    "#8b5cf6",
    "#ec4899",
    "#06b6d4",
  ];

  const resetFilters = () => {
    setSearchTerm("");
    setAppFilter("All Apps");
    setStartTimeFilter("00:00");
    setEndTimeFilter("23:59");
  };

  const chartData = useMemo(() => screentime.slice(0, 10), [screentime]);

  return (
    <div className="min-h-screen bg-neutral-950 text-neutral-100 font-sans">
      {/* Header */}
      <header className="border-b border-neutral-800 bg-neutral-900/50 backdrop-blur-md sticky top-0 z-20">
        <div className="max-w-7xl mx-auto px-4 h-16 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
              <Clock className="w-5 h-5 text-white" />
            </div>
            <h1 className="text-xl font-bold tracking-tight">waysted</h1>
          </div>

          <div
            className="relative flex items-center gap-2 bg-neutral-800 rounded-full px-2 py-1"
            ref={calendarRef}
          >
            <button
              onClick={() => setSelectedDate(subDays(selectedDate, 1))}
              className="p-1.5 hover:bg-neutral-700 rounded-full transition-colors"
            >
              <ChevronLeft className="w-5 h-5" />
            </button>
            <button
              onClick={() => setShowCalendar(!showCalendar)}
              className="flex items-center gap-2 px-3 py-1 hover:bg-neutral-700 rounded-full transition-colors group"
            >
              <CalendarIcon className="w-4 h-4 text-neutral-400 group-hover:text-blue-400" />
              <span className="font-medium min-w-[120px] text-center">
                {isSameDay(selectedDate, new Date())
                  ? "Today"
                  : format(selectedDate, "MMM d, yyyy")}
              </span>
              <ChevronDown
                className={cn(
                  "w-4 h-4 text-neutral-500 transition-transform",
                  showCalendar && "rotate-180",
                )}
              />
            </button>
            <button
              onClick={() => setSelectedDate(addDays(selectedDate, 1))}
              className="p-1.5 hover:bg-neutral-700 rounded-full transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
              disabled={isSameDay(selectedDate, new Date())}
            >
              <ChevronRight className="w-5 h-5" />
            </button>

            {showCalendar && (
              <div className="absolute top-full mt-2 left-1/2 -translate-x-1/2 bg-neutral-900 border border-neutral-800 rounded-2xl p-4 shadow-2xl z-30">
                <DayPicker
                  mode="single"
                  selected={selectedDate}
                  onSelect={(date) => {
                    if (date) {
                      setSelectedDate(date);
                      setShowCalendar(false);
                    }
                  }}
                  disabled={{ after: new Date() }}
                  className="rdp-custom"
                />
              </div>
            )}
          </div>

          <nav className="flex items-center gap-1 bg-neutral-800/50 p-1 rounded-xl">
            {[
              { id: "dashboard", icon: BarChart3, label: "Dashboard" },
              { id: "schedule", icon: Clock, label: "Schedule" },
              { id: "logs", icon: List, label: "Logs" },
            ].map((item) => (
              <button
                key={item.id}
                onClick={() =>
                  setView(item.id as "dashboard" | "logs" | "schedule")
                }
                className={cn(
                  "flex items-center gap-2 px-4 py-2 rounded-lg transition-all text-sm font-medium",
                  view === item.id
                    ? "bg-blue-600 text-white shadow-lg shadow-blue-900/20"
                    : "text-neutral-400 hover:text-neutral-100 hover:bg-neutral-800",
                )}
              >
                <item.icon className="w-4 h-4" />
                {item.label}
              </button>
            ))}
          </nav>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 py-8">
        {loading ? (
          <div className="flex flex-col items-center justify-center h-[60vh] gap-4">
            <div className="w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
            <p className="text-neutral-400 animate-pulse">
              Loading your data...
            </p>
          </div>
        ) : screentime.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-[60vh] text-center">
            <div className="w-20 h-20 bg-neutral-900 rounded-3xl flex items-center justify-center mb-6 border border-neutral-800">
              <Monitor className="w-10 h-10 text-neutral-600" />
            </div>
            <h2 className="text-2xl font-semibold mb-2">No activity found</h2>
            <p className="text-neutral-400 max-w-md">
              We couldn&apos;t find any screentime data for{" "}
              {format(selectedDate, "MMMM d, yyyy")}. Make sure the waysted
              daemon is running.
            </p>
          </div>
        ) : (
          <div className="space-y-8">
            {/* Summary Stats */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="bg-neutral-900 border border-neutral-800 p-6 rounded-2xl transition-all hover:border-neutral-700">
                <p className="text-neutral-400 text-sm font-medium mb-1">
                  Total Screentime
                </p>
                <h3 className="text-3xl font-bold">
                  {formatDuration(totalDuration)}
                </h3>
              </div>
              <div className="bg-neutral-900 border border-neutral-800 p-6 rounded-2xl transition-all hover:border-neutral-700">
                <p className="text-neutral-400 text-sm font-medium mb-1">
                  Most Used App
                </p>
                <h3 className="text-3xl font-bold truncate">
                  {screentime[0]?.app_name || "N/A"}
                </h3>
              </div>
              <div className="bg-neutral-900 border border-neutral-800 p-6 rounded-2xl transition-all hover:border-neutral-700">
                <p className="text-neutral-400 text-sm font-medium mb-1">
                  Active Windows
                </p>
                <h3 className="text-3xl font-bold">{logs.length}</h3>
              </div>
            </div>

            {view === "dashboard" && (
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                {/* Chart */}
                <div className="bg-neutral-900 border border-neutral-800 p-6 rounded-2xl flex flex-col">
                  <h4 className="text-lg font-semibold mb-6 flex items-center gap-2">
                    <BarChart3 className="w-5 h-5 text-blue-500" />
                    App Usage Breakdown
                  </h4>
                  <div className="h-[400px] w-full">
                    <ResponsiveContainer width="100%" height="100%">
                      <BarChart
                        data={chartData}
                        layout="vertical"
                        margin={{ left: 20, right: 20 }}
                      >
                        <CartesianGrid
                          strokeDasharray="3 3"
                          stroke="#333"
                          horizontal={false}
                        />
                        <XAxis type="number" hide />
                        <YAxis
                          dataKey="app_name"
                          type="category"
                          stroke="#999"
                          fontSize={12}
                          width={120}
                        />
                        <Tooltip
                          cursor={{ fill: "#222" }}
                          contentStyle={{
                            backgroundColor: "#171717",
                            border: "1px solid #333",
                            borderRadius: "12px",
                          }}
                          formatter={(value: TooltipValueType | undefined) => [
                            typeof value === "number"
                              ? formatDuration(value)
                              : "0s",
                            "Duration",
                          ]}
                        />
                        <Bar
                          dataKey="duration"
                          radius={[0, 4, 4, 0]}
                          shape={(props: BarShapeProps) => {
                            const { index } = props;
                            return (
                              <Rectangle
                                {...props}
                                fill={COLORS[index % COLORS.length]}
                              />
                            );
                          }}
                        />
                      </BarChart>
                    </ResponsiveContainer>
                  </div>
                </div>

                {/* Top Apps List */}
                <div className="bg-neutral-900 border border-neutral-800 p-6 rounded-2xl overflow-hidden flex flex-col">
                  <h4 className="text-lg font-semibold mb-6 flex items-center gap-2">
                    <List className="w-5 h-5 text-emerald-500" />
                    Top Applications
                  </h4>
                  <div className="space-y-4 overflow-y-auto max-h-[400px] pr-2 custom-scrollbar">
                    {screentime.map((app, idx) => (
                      <div
                        key={app.id || app.app_name}
                        className="group flex items-center justify-between p-3 rounded-xl hover:bg-neutral-800/50 transition-colors"
                      >
                        <div className="flex items-center gap-4">
                          <div
                            className="w-10 h-10 rounded-lg flex items-center justify-center text-lg font-bold"
                            style={{
                              backgroundColor: `${COLORS[idx % COLORS.length]}20`,
                              color: COLORS[idx % COLORS.length],
                            }}
                          >
                            {app.app_name[0]}
                          </div>
                          <div>
                            <p className="font-semibold">{app.app_name}</p>
                            <p className="text-xs text-neutral-500">
                              {app.percentage}% of total time
                            </p>
                          </div>
                        </div>
                        <div className="text-right">
                          <p className="font-mono font-medium">
                            {formatShortDuration(app.duration)}
                          </p>
                          <div className="w-24 h-1.5 bg-neutral-800 rounded-full mt-1 overflow-hidden">
                            <div
                              className="h-full rounded-full"
                              style={{
                                width: `${app.percentage}%`,
                                backgroundColor: COLORS[idx % COLORS.length],
                              }}
                            />
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            )}

            {view === "schedule" && (
              <div className="bg-neutral-900 border border-neutral-800 rounded-2xl p-8">
                <h4 className="text-xl font-semibold mb-8 flex items-center gap-2">
                  <Clock className="w-6 h-6 text-blue-500" />
                  Hourly Activity Timeline
                </h4>
                <div className="space-y-6">
                  {hourlyData.map((hour) => (
                    <div key={hour.hour} className="flex gap-6 group">
                      <div className="w-16 pt-1 text-sm font-mono text-neutral-500 text-right shrink-0">
                        {hour.label}
                      </div>
                      <div className="relative grow pb-6 border-l-2 border-neutral-800 pl-8">
                        <div className="absolute -left-[9px] top-2 w-4 h-4 rounded-full bg-neutral-800 border-2 border-neutral-950 group-hover:border-blue-500 transition-colors" />

                        {hour.duration > 0 ? (
                          <div className="space-y-3">
                            <div className="flex items-center gap-3">
                              <span className="text-lg font-bold">
                                {formatDuration(hour.duration)}
                              </span>
                              <span className="text-xs px-2 py-0.5 bg-neutral-800 rounded-full text-neutral-400">
                                {Object.keys(hour.apps).length} apps used
                              </span>
                            </div>
                            <div className="flex flex-wrap gap-2">
                              {Object.entries(hour.apps)
                                .sort((a, b) => b[1] - a[1])
                                .slice(0, 5)
                                .map(([name, dur], i) => (
                                  <div
                                    key={name}
                                    className="flex items-center gap-2 bg-neutral-800/80 px-3 py-1.5 rounded-lg border border-neutral-700"
                                  >
                                    <div
                                      className="w-2 h-2 rounded-full"
                                      style={{
                                        backgroundColor:
                                          COLORS[i % COLORS.length],
                                      }}
                                    />
                                    <span className="text-sm font-medium">
                                      {name}
                                    </span>
                                    <span className="text-xs text-neutral-500 font-mono">
                                      {formatShortDuration(dur)}
                                    </span>
                                  </div>
                                ))}
                            </div>
                          </div>
                        ) : (
                          <p className="text-neutral-600 italic text-sm py-1">
                            No activity logged
                          </p>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {view === "logs" && (
              <div className="bg-neutral-900 border border-neutral-800 rounded-2xl overflow-hidden flex flex-col">
                <div className="p-6 border-b border-neutral-800 space-y-4">
                  <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
                    <h4 className="text-lg font-semibold flex items-center gap-2">
                      <List className="w-5 h-5 text-purple-500" />
                      Detailed Logs
                    </h4>
                    <button
                      onClick={resetFilters}
                      className="text-xs text-neutral-500 hover:text-neutral-300 flex items-center gap-1 transition-colors"
                    >
                      <X className="w-3 h-3" />
                      Clear Filters
                    </button>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    {/* Search */}
                    <div className="relative">
                      <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-500" />
                      <input
                        type="text"
                        placeholder="Search window title..."
                        className="w-full bg-neutral-800 border border-neutral-700 rounded-xl py-2 pl-10 pr-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 transition-all"
                        value={searchTerm}
                        onChange={(e) => setSearchTerm(e.target.value)}
                      />
                    </div>

                    {/* App Filter */}
                    <div className="relative">
                      <Filter className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-500" />
                      <select
                        className="w-full bg-neutral-800 border border-neutral-700 rounded-xl py-2 pl-10 pr-4 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 appearance-none cursor-pointer"
                        value={appFilter}
                        onChange={(e) => setAppFilter(e.target.value)}
                      >
                        {appNames.map((name) => (
                          <option key={name} value={name}>
                            {name}
                          </option>
                        ))}
                      </select>
                    </div>

                    {/* Time Start */}
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-neutral-500 shrink-0">
                        From
                      </span>
                      <input
                        type="time"
                        className="w-full bg-neutral-800 border border-neutral-700 rounded-xl py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50"
                        value={startTimeFilter}
                        onChange={(e) => setStartTimeFilter(e.target.value)}
                      />
                    </div>

                    {/* Time End */}
                    <div className="flex items-center gap-2">
                      <span className="text-xs text-neutral-500 shrink-0">
                        To
                      </span>
                      <input
                        type="time"
                        className="w-full bg-neutral-800 border border-neutral-700 rounded-xl py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50"
                        value={endTimeFilter}
                        onChange={(e) => setEndTimeFilter(e.target.value)}
                      />
                    </div>
                  </div>

                  <div className="text-xs text-neutral-500">
                    Showing {filteredLogs.length} of {logs.length} entries
                  </div>
                </div>
                <div className="overflow-x-auto">
                  <table className="w-full text-left border-collapse">
                    <thead>
                      <tr className="bg-neutral-950/50 text-neutral-400 text-xs uppercase tracking-wider">
                        <th className="px-6 py-4 font-semibold">Time</th>
                        <th className="px-6 py-4 font-semibold">App</th>
                        <th className="px-6 py-4 font-semibold">
                          Window Title
                        </th>
                        <th className="px-6 py-4 font-semibold text-right">
                          Duration
                        </th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-neutral-800">
                      {filteredLogs.map((log) => (
                        <tr
                          key={log.id}
                          className="hover:bg-neutral-800/30 transition-colors group"
                        >
                          <td className="px-6 py-4 text-sm font-mono text-neutral-400 whitespace-nowrap">
                            {format(new Date(log.start_timestamp), "HH:mm:ss")}
                          </td>
                          <td className="px-6 py-4">
                            <span className="px-2 py-1 bg-blue-900/20 text-blue-400 rounded text-[10px] font-bold border border-blue-900/30">
                              {log.app_name}
                            </span>
                          </td>
                          <td
                            className="px-6 py-4 text-sm max-w-md truncate text-neutral-300"
                            title={log.title}
                          >
                            {log.title}
                          </td>
                          <td className="px-6 py-4 text-sm font-mono text-right font-medium">
                            {formatDuration(log.duration)}
                          </td>
                        </tr>
                      ))}
                      {filteredLogs.length === 0 && (
                        <tr>
                          <td
                            colSpan={4}
                            className="px-6 py-12 text-center text-neutral-500 italic"
                          >
                            No logs found matching your filters.
                          </td>
                        </tr>
                      )}
                    </tbody>
                  </table>
                </div>
              </div>
            )}
          </div>
        )}
      </main>

      <style jsx global>{`
        .custom-scrollbar::-webkit-scrollbar {
          width: 6px;
        }
        .custom-scrollbar::-webkit-scrollbar-track {
          background: transparent;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb {
          background: #333;
          border-radius: 10px;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover {
          background: #444;
        }

        /* react-day-picker dark theme overrides */
        .rdp-custom {
          --rdp-cell-size: 40px;
          --rdp-accent-color: #3b82f6;
          --rdp-background-color: #2563eb;
          --rdp-outline: 2px solid var(--rdp-accent-color);
          --rdp-outline-offset: 2px;
          --rdp-selected-color: #fff;
          margin: 0;
        }
        .rdp-selected,
        .rdp-selected:focus,
        .rdp-selected:hover {
          background-color: var(--rdp-accent-color) !important;
          color: white !important;
        }
        .rdp-button:hover:not([disabled]):not(.rdp-selected) {
          background-color: #262626;
        }
      `}</style>
    </div>
  );
}
