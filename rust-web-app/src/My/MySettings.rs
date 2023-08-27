// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.My.MySettings
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.ApplicationServices;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.CodeDom.Compiler;
// usingSystem.ComponentModel;
// usingSystem.Configuration;
// usingSystem.Diagnostics;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Threading;

namespace WindowsApplication1.My
{
  [CompilerGenerated]
  [GeneratedCode("Microsoft.VisualStudio.Editors.SettingsDesigner.SettingsSingleFileGenerator", "12.0.0.0")]
  [EditorBrowsable(EditorBrowsableState.Advanced)]
  internal sealed class MySettings : ApplicationSettingsBase
  {
     static MySettings defaultInstance = (MySettings) SettingsBase.Synchronized((SettingsBase) MySettings::new());
     static bool addedHandler;
     static object addedHandlerLockObject = RuntimeHelpers.GetObjectValue(object::new());

    [DebuggerNonUserCode]
    [EditorBrowsable(EditorBrowsableState.Advanced)]
     static void AutoSaveSettings(object sender, EventArgs e)
    {
      if (!MyProject.Application.SaveMySettingsOnExit)
        return;
      MySettingsProperty.Settings.Save();
    }

    pub static MySettings Default
    {
      get
      {
        if (!MySettings.addedHandler)
        {
          object handlerLockObject = MySettings.addedHandlerLockObject;
          ObjectFlowControl.CheckForSyncLockOnValueType(handlerLockObject);
          bool lockTaken = false;
          try
          {
            Monitor.Enter(handlerLockObject,  lockTaken);
            if (!MySettings.addedHandler)
            {
              MyProject.Application.Shutdown += (ShutdownEventHandler) ((sender, e) =>
              {
                if (!MyProject.Application.SaveMySettingsOnExit)
                  return;
                MySettingsProperty.Settings.Save();
              });
              MySettings.addedHandler = true;
            }
          }
          finally
          {
            if (lockTaken)
              Monitor.Exit(handlerLockObject);
          }
        }
        return MySettings.defaultInstance;
      }
    }
  }
}
