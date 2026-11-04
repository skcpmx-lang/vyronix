import { Mail, User, Package, Scale, Cpu } from "lucide-react";
import { useAsync } from "../hooks/useAsync";
import { getAppInfo } from "../lib/services";
import { Card, PageHeader, KeyValue } from "../components/Card";
import { VxLogo } from "../components/Sidebar";
import { Spinner } from "../components/ProgressBar";

const CREDITS = [
  { name: "Tauri", purpose: "Secure, lightweight desktop framework", license: "MIT / Apache-2.0" },
  { name: "React", purpose: "User interface library", license: "MIT" },
  { name: "sysinfo", purpose: "Cross-platform system information", license: "MIT" },
  { name: "starship-battery", purpose: "Battery information interface", license: "MIT" },
  { name: "wmi", purpose: "Windows Management Instrumentation access", license: "MIT" },
  { name: "rusqlite", purpose: "SQLite persistence", license: "MIT" },
  { name: "jsPDF", purpose: "Offline PDF report generation", license: "MIT" },
  { name: "Tailwind CSS", purpose: "Utility-first styling", license: "MIT" },
  { name: "Lucide", purpose: "Icon set", license: "ISC" },
  { name: "Zustand", purpose: "State management", license: "MIT" },
];

export function About() {
  const info = useAsync(getAppInfo, []);

  return (
    <div>
      <PageHeader title="About VYRONIX" description="Product identity, version and license information" />

      <Card className="mb-4">
        <div className="flex items-center gap-4">
          <VxLogo size={56} />
          <div>
            <div className="text-xs tracking-[0.3em] text-vx-500 font-semibold">MERQO SOFTWARE PRODUCT</div>
            <div className="text-2xl font-bold">VYRONIX</div>
            <div className="text-sm text-[var(--vx-muted)]">Know Your Hardware. Trust Your Device.</div>
          </div>
        </div>
      </Card>

      {info.loading ? <Spinner /> : info.data && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <Card title="Product">
            <KeyValue label="Product brand" value={info.data.brand} />
            <KeyValue label="Product name" value={info.data.name} />
            <KeyValue label="Product type" value={info.data.product_type} />
            <KeyValue label="Application version" value={info.data.version} mono />
            <KeyValue label="Build number" value={info.data.build_number} mono />
            <KeyValue label="Architecture" value={info.data.architecture} />
          </Card>

          <Card title="Owner & contact">
            <div className="space-y-3">
              <div className="flex items-center gap-3">
                <User className="w-4 h-4 text-vx-500" />
                <div>
                  <div className="vx-label">Founder</div>
                  <div className="text-sm font-medium">{info.data.founder}</div>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <Mail className="w-4 h-4 text-vx-500" />
                <div>
                  <div className="vx-label">Email</div>
                  <a href={`mailto:${info.data.email}`} className="text-sm font-medium text-vx-500 hover:underline">{info.data.email}</a>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <Scale className="w-4 h-4 text-vx-500" />
                <div>
                  <div className="vx-label">Copyright</div>
                  <div className="text-sm">{info.data.copyright}</div>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <Package className="w-4 h-4 text-vx-500" />
                <div>
                  <div className="vx-label">License</div>
                  <div className="text-sm">{info.data.license}</div>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <Cpu className="w-4 h-4 text-vx-500" />
                <div>
                  <div className="vx-label">Target platform</div>
                  <div className="text-sm">Windows 10 / 11 (64-bit) · unsigned distribution</div>
                </div>
              </div>
            </div>
          </Card>

          <Card title="Software credits & third-party dependencies" className="lg:col-span-2">
            <p className="text-xs text-[var(--vx-muted)] mb-3">
              VYRONIX builds on the following open-source components. Their licenses are respected and
              reproduced in the distribution. VYRONIX itself is proprietary MERQO software.
            </p>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="text-left text-xs text-[var(--vx-muted)] border-b border-[var(--vx-border)]">
                    <th className="py-2 pr-4 font-semibold">Component</th>
                    <th className="py-2 pr-4 font-semibold">Purpose</th>
                    <th className="py-2 font-semibold">License</th>
                  </tr>
                </thead>
                <tbody>
                  {CREDITS.map((c) => (
                    <tr key={c.name} className="border-b border-[var(--vx-border)] last:border-0">
                      <td className="py-2 pr-4 font-medium">{c.name}</td>
                      <td className="py-2 pr-4 text-[var(--vx-muted)]">{c.purpose}</td>
                      <td className="py-2 text-[var(--vx-muted)]">{c.license}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </Card>
        </div>
      )}
    </div>
  );
}
