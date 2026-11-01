export type Locale = "zh" | "en";

const translations: Record<Locale, Record<string, string>> = {
  zh: {
    // Common
    "app.name": "BitClock 比特时钟",
    "app.description": "像素风使用时长追踪器",

    // Navigation
    "nav.home": "首页",
    "nav.history": "历史统计",
    "nav.settings": "设置",

    // Home
    "home.today_usage": "今日使用",
    "home.loading": "加载中...",
    "home.refresh": "刷新数据",
    "home.duration_prefix": "已持续：",
    "home.current_app": "当前应用",
    "home.app_ranking": "应用排行",
    "home.app_count": "个应用",
    "home.no_data": "暂无数据",
    "home.view_history": "查看历史统计 →",
    "home.daily_goal": "每日目标",
    "home.goal_exceeded": "目标达成！",
    "home.goal_exceeded_text": "今日使用时间已达到 {minutes} 分钟的目标。",
    "home.goal_exceeded_sub": "注意适当休息，保护眼睛哦！",
    "home.goal_approaching": "即将达到每日目标，请注意休息",
    "home.break_reminder": "该休息了！",
    "home.break_reminder_text": "已经连续使用电脑一段时间了。",
    "home.break_reminder_sub": "站起来活动一下，看看远处，保护眼睛和颈椎！",
    "home.snooze": "稍后提醒",
    "home.i_took_break": "我休息了",
    "home.known": "知道了",
    "home.weekly_trend": "近 7 天",
    "home.usage_distribution": "使用分布",
    "home.category_stats": "分类统计",
    "home.timeline": "24 小时分布",
    "home.click_app_detail": "点击查看详情",
    "home.weekly_report": "上周使用 {total}，日均 {avg}",
    "home.paused": "追踪已暂停",
    "home.resume": "恢复",
    "home.pinned": "窗口已置顶",
    "home.unpin": "取消",

    // Pomodoro
    "pomodoro.title": "番茄钟",
    "pomodoro.work": "工作中",
    "pomodoro.break": "短休息",
    "pomodoro.long_break": "长休息",
    "pomodoro.start": "开始",
    "pomodoro.stop": "停止",
    "pomodoro.skip": "跳过",
    "pomodoro.cycle": "第 {n} 个番茄",
    "pomodoro.work_start": "番茄开始！专注工作吧。",
    "pomodoro.break_start": "休息一下！站起来活动活动。",
    "pomodoro.long_break_start": "长休息时间！好好放松一下。",

    // History
    "history.title": "历史统计",
    "history.this_week": "本周",
    "history.this_month": "本月",
    "history.total": "总计",
    "history.daily_avg": "日均",
    "history.longest_day": "最长单日",
    "history.active_days": "活跃天数",
    "history.top_apps": "热门应用",
    "history.daily_avg_pct": "日均 {pct}%",
    "history.comparison": "对比分析",
    "history.this_week_vs_last": "本周 vs 上周",
    "history.this_month_vs_last": "本月 vs 上月",
    "history.change_pct": "变化 {pct}%",
    "history.increase": "↑ 增加",
    "history.decrease": "↓ 减少",
    "history.no_change": "— 持平",

    // Settings
    "settings.title": "设置",
    "settings.appearance": "外观",
    "settings.theme": "主题",
    "settings.dark": "暗色",
    "settings.light": "亮色",
    "settings.auto": "自动",
    "settings.accent_color": "强调色",
    "settings.reset": "重置",
    "settings.default": "默认",
    "settings.startup": "启动",
    "settings.autostart": "开机自启动",
    "settings.reminder": "提醒",
    "settings.exit_reminder": "退出时显示使用总结",
    "settings.idle_detection": "空闲检测",
    "settings.idle_threshold": "空闲判定时间",
    "settings.minutes": "分钟",
    "settings.break_reminder": "休息提醒",
    "settings.pomodoro": "番茄钟",
    "settings.pomodoro_enable": "启用番茄钟",
    "settings.pomodoro_work": "工作时长",
    "settings.pomodoro_break": "短休息",
    "settings.pomodoro_long_break": "长休息",
    "settings.pomodoro_cycles": "长休息间隔",
    "settings.pomodoro_cycles_hint": "每 {count} 个番茄后长休息",
    "settings.break_interval": "提醒间隔",
    "settings.daily_goal": "每日目标",
    "settings.usage_limit": "使用时长上限",
    "settings.disable_hint": "设为 0 关闭",
    "settings.data": "数据",
    "settings.retention_days": "保留天数",
    "settings.days": "天",
    "settings.auto_cleanup": "自动清理 {days} 天前的数据",
    "settings.cleanup_now": "立即清理",
    "settings.confirm_cleanup": "确认清理？",
    "settings.export_records": "导出全部使用记录",
    "settings.exporting": "导出中...",
    "settings.backup_db": "备份数据库文件",
    "settings.backing": "备份中...",
    "settings.backup": "备份数据",
    "settings.restore_hint": "从备份文件恢复（需重启）",
    "settings.restore": "恢复数据",
    "settings.categories": "应用分类",
    "settings.categories_hint": "为应用设置分类，首页可查看分类统计",
    "settings.categories_empty": "暂无应用分类",
    "settings.about": "关于",
    "settings.version": "版本",
    "settings.saving": "保存中...",
    "settings.cleanup_done": "清理完成",
    "settings.cleanup_result": "已清理 {count} 条旧记录",
    "settings.export_done": "导出完成，共 {count} 条记录",
    "settings.click_to_close": "点击关闭",
    "settings.language": "语言 / Language",
    "settings.backup_done": "备份完成",
    "settings.restore_confirm": "恢复数据将覆盖当前数据，需要重启应用才能生效。确定继续吗？",
    "settings.restore_done": "数据已恢复，请重启应用以生效。",

    // App Detail
    "app_detail.no_title": "无标题",
    "app_detail.daily_trend": "每日趋势",
    "app_detail.history": "使用历史",
    "app_detail.window_titles": "窗口标题",

    // Categories
    "category.work": "工作",
    "category.entertainment": "娱乐",
    "category.social": "社交",
    "category.development": "开发",
    "category.design": "设计",
    "category.other": "其他",

    // Exit reminder
    "exit.title": "■ BitClock 比特时钟",
    "exit.greeting": "今天辛苦了！",
    "exit.greeting_alt1": "干得不错！",
    "exit.greeting_alt2": "又是充实的一天！",
    "exit.greeting_alt3": "努力工作的一天！",
    "exit.greeting_alt4": "效率满满！",
    "exit.usage_summary": "你今天使用了电脑",
    "exit.most_used": "最常用：",
    "exit.app_count": "应用数：",
    "exit.apps": "个",
    "exit.tip": "记得休息眼睛哦~",
    "exit.cancel": "取消",
    "exit.ok": "知道了",

    // Modal
    "modal.default_label": "对话框",
    "modal.close": "关闭",

    // Errors
    "error.load_settings": "设置加载失败",
    "error.save_settings": "设置保存失败",
    "error.registry": "注册表修改失败，开机自启动可能不生效",
    "error.export": "导出失败",
    "error.backup": "备份失败",
    "error.restore": "恢复失败",
  },

  en: {
    // Common
    "app.name": "BitClock",
    "app.description": "Pixel-style usage tracker",

    // Navigation
    "nav.home": "Home",
    "nav.history": "History",
    "nav.settings": "Settings",

    // Home
    "home.today_usage": "Today's Usage",
    "home.loading": "Loading...",
    "home.refresh": "Refresh",
    "home.duration_prefix": "Duration: ",
    "home.current_app": "Current App",
    "home.app_ranking": "App Ranking",
    "home.app_count": "apps",
    "home.no_data": "No data yet",
    "home.view_history": "View History →",
    "home.daily_goal": "Daily Goal",
    "home.goal_exceeded": "Goal Reached!",
    "home.goal_exceeded_text": "You've reached your {minutes}-minute daily target.",
    "home.goal_exceeded_sub": "Remember to take breaks and protect your eyes!",
    "home.goal_approaching": "Approaching daily goal, take a break",
    "home.break_reminder": "Time for a Break!",
    "home.break_reminder_text": "You've been using the computer for a while.",
    "home.break_reminder_sub": "Stand up, stretch, and rest your eyes!",
    "home.snooze": "Snooze",
    "home.i_took_break": "I Took a Break",
    "home.known": "Got It",
    "home.weekly_trend": "Last 7 Days",
    "home.usage_distribution": "Usage Distribution",
    "home.category_stats": "Category Stats",
    "home.timeline": "24h Distribution",
    "home.click_app_detail": "Click for details",
    "home.weekly_report": "Last week: {total}, daily avg: {avg}",
    "home.paused": "Tracking Paused",
    "home.resume": "Resume",
    "home.pinned": "Window Pinned",
    "home.unpin": "Unpin",

    // Pomodoro
    "pomodoro.title": "Pomodoro",
    "pomodoro.work": "Working",
    "pomodoro.break": "Short Break",
    "pomodoro.long_break": "Long Break",
    "pomodoro.start": "Start",
    "pomodoro.stop": "Stop",
    "pomodoro.skip": "Skip",
    "pomodoro.cycle": "Pomodoro #{n}",
    "pomodoro.work_start": "Pomodoro started! Focus on your work.",
    "pomodoro.break_start": "Break time! Stand up and stretch.",
    "pomodoro.long_break_start": "Long break! Relax and recharge.",

    // History
    "history.title": "History",
    "history.this_week": "This Week",
    "history.this_month": "This Month",
    "history.total": "Total",
    "history.daily_avg": "Daily Avg",
    "history.longest_day": "Longest Day",
    "history.active_days": "Active Days",
    "history.top_apps": "Top Apps",
    "history.daily_avg_pct": "Daily Avg {pct}%",
    "history.comparison": "Comparison",
    "history.this_week_vs_last": "This Week vs Last",
    "history.this_month_vs_last": "This Month vs Last",
    "history.change_pct": "{pct}% change",
    "history.increase": "↑ Increased",
    "history.decrease": "↓ Decreased",
    "history.no_change": "— No change",

    // Settings
    "settings.title": "Settings",
    "settings.appearance": "Appearance",
    "settings.theme": "Theme",
    "settings.dark": "Dark",
    "settings.light": "Light",
    "settings.auto": "Auto",
    "settings.accent_color": "Accent Color",
    "settings.reset": "Reset",
    "settings.default": "Default",
    "settings.startup": "Startup",
    "settings.autostart": "Launch at startup",
    "settings.reminder": "Reminder",
    "settings.exit_reminder": "Show usage summary on exit",
    "settings.idle_detection": "Idle Detection",
    "settings.idle_threshold": "Idle threshold",
    "settings.minutes": "min",
    "settings.break_reminder": "Break Reminder",
    "settings.pomodoro": "Pomodoro",
    "settings.pomodoro_enable": "Enable Pomodoro",
    "settings.pomodoro_work": "Work Duration",
    "settings.pomodoro_break": "Short Break",
    "settings.pomodoro_long_break": "Long Break",
    "settings.pomodoro_cycles": "Long Break Interval",
    "settings.pomodoro_cycles_hint": "Long break every {count} pomodoros",
    "settings.break_interval": "Reminder interval",
    "settings.daily_goal": "Daily Goal",
    "settings.usage_limit": "Usage limit",
    "settings.disable_hint": "Set to 0 to disable",
    "settings.data": "Data",
    "settings.retention_days": "Retention",
    "settings.days": "days",
    "settings.auto_cleanup": "Auto-cleanup data older than {days} days",
    "settings.cleanup_now": "Cleanup Now",
    "settings.confirm_cleanup": "Confirm?",
    "settings.export_records": "Export all records",
    "settings.exporting": "Exporting...",
    "settings.backup_db": "Backup database",
    "settings.backing": "Backing up...",
    "settings.backup": "Backup",
    "settings.restore_hint": "Restore from backup (requires restart)",
    "settings.restore": "Restore",
    "settings.categories": "App Categories",
    "settings.categories_hint": "Set categories for apps to view category stats on home page",
    "settings.categories_empty": "No app categories yet",
    "settings.about": "About",
    "settings.version": "Version",
    "settings.saving": "Saving...",
    "settings.cleanup_done": "Cleanup Done",
    "settings.cleanup_result": "Cleaned up {count} old records",
    "settings.export_done": "Export complete, {count} records total",
    "settings.click_to_close": "click to close",
    "settings.language": "Language",
    "settings.backup_done": "Backup complete",
    "settings.restore_confirm": "Restoring will overwrite current data. Restart required. Continue?",
    "settings.restore_done": "Data restored. Please restart the app.",

    // App Detail
    "app_detail.no_title": "(No title)",
    "app_detail.daily_trend": "Daily Trend",
    "app_detail.history": "Usage History",
    "app_detail.window_titles": "Window Titles",

    // Categories
    "category.work": "Work",
    "category.entertainment": "Entertainment",
    "category.social": "Social",
    "category.development": "Development",
    "category.design": "Design",
    "category.other": "Other",

    // Exit reminder
    "exit.title": "■ BitClock",
    "exit.greeting": "Good work today!",
    "exit.greeting_alt1": "Nice job!",
    "exit.greeting_alt2": "Another productive day!",
    "exit.greeting_alt3": "Hard working day!",
    "exit.greeting_alt4": "Full of efficiency!",
    "exit.usage_summary": "You used the computer for",
    "exit.most_used": "Most used: ",
    "exit.app_count": "Apps: ",
    "exit.apps": "",
    "exit.tip": "Remember to rest your eyes~",
    "exit.cancel": "Cancel",
    "exit.ok": "Got It",

    // Modal
    "modal.default_label": "Dialog",
    "modal.close": "Close",

    // Errors
    "error.load_settings": "Failed to load settings",
    "error.save_settings": "Failed to save settings",
    "error.registry": "Registry error, auto-start may not work",
    "error.export": "Export failed",
    "error.backup": "Backup failed",
    "error.restore": "Restore failed",
  },
};

const raw = localStorage.getItem("bitclock-locale");
// $state makes locale changes reactive — components calling t() re-render automatically
export let currentLocale: Locale = $state(raw === "en" ? "en" : "zh");

export function getLocale(): Locale {
  return currentLocale;
}

export function setLocale(locale: Locale) {
  currentLocale = locale;
  localStorage.setItem("bitclock-locale", locale);
}

/**
 * Translate a key to the current locale's text.
 * Because currentLocale is $state, any component calling t() during render
 * will automatically re-render when the locale changes.
 */
export function t(key: string, params?: Record<string, string | number>): string {
  // Read currentLocale to establish a reactive dependency
  const locale = currentLocale;
  let text = translations[locale]?.[key] || translations["zh"]?.[key] || key;
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replace(`{${k}}`, String(v));
    }
  }
  return text;
}
