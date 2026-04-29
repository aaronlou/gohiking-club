import { User } from "lucide-react";

// Placeholder — will be connected to real auth later
export default function Profile() {
  return (
    <div className="mx-auto max-w-lg text-center">
      <div className="mb-6 inline-flex h-24 w-24 items-center justify-center rounded-full bg-gray-100">
        <User className="h-12 w-12 text-gray-400" />
      </div>
      <h1 className="mb-2 text-2xl font-bold text-gray-900">个人中心</h1>
      <p className="text-gray-500">
        用户系统即将上线，敬请期待。
      </p>
    </div>
  );
}
