use std::path::Display;
use windows::{
    Win32::{
        NetworkManagement::WindowsFirewall::*,
        System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
    },
};
use windows::core::{BSTR, *};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};
use windows::Win32::Foundation::VARIANT_BOOL;

pub fn add_rules(name: String, dir: &&str, display: Display) -> Result<()> {
    unsafe {
        CoInitializeEx(Some(std::ptr::null()), COINIT_APARTMENTTHREADED)?;

        let manager: INetFwPolicy2 = CoCreateInstance(&NetFwPolicy2, None, CLSCTX_ALL)?;

        let rule: INetFwRule = CoCreateInstance(&NetFwRule, None, CLSCTX_ALL)?;
        rule.SetName(&BSTR::from(name))?;
        rule.SetDescription(&BSTR::from("Description of the rule"))?;

        rule.SetProtocol(NET_FW_IP_PROTOCOL_ANY.0)?;

        // rule.SetLocalPorts(&BSTR::from("8080"))?;
        if *dir == "in" {
            rule.SetDirection(NET_FW_RULE_DIR_IN)?;
        }else {
            rule.SetDirection(NET_FW_RULE_DIR_OUT)?;
        }
        rule.SetAction(NET_FW_ACTION_BLOCK)?;
        rule.SetEnabled(VARIANT_BOOL::from(true))?;
        // 设置程序路径
        rule.SetApplicationName(&BSTR::from(display.to_string()))?;
        // 这里的例子将规则应用于域、专用和公用网络
        // 将枚举值转换为其整数表示，然后进行位运算
        let profiles = NET_FW_PROFILE2_DOMAIN.0 | NET_FW_PROFILE2_PRIVATE.0 | NET_FW_PROFILE2_PUBLIC.0;
        rule.SetProfiles(profiles as i32)?;
        let rules = manager.Rules()?;
        rules.Add(&rule)?;


        println!("规则已添加");

        Ok(())
    }
}

pub fn delete_rules(rule_name: &str) -> Result<()> {
    unsafe {
        CoInitializeEx(Some(std::ptr::null()), COINIT_APARTMENTTHREADED)?;

        let manager: INetFwPolicy2 = CoCreateInstance(&NetFwPolicy2, None, CLSCTX_ALL)?;

        let rules = manager.Rules()?;
        rules.Remove(&BSTR::from(rule_name))?;

        println!("规则 '{}' 已删除", rule_name);

        Ok(())
    }
}
