// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.DEVMODE1
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem.Runtime.InteropServices;

namespace WindowsApplication1
{
  pub struct DEVMODE1
  {
    [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
    pub dmDeviceName: String;
    pub short dmSpecVersion;
    pub short dmDriverVersion;
    pub short dmSize;
    pub short dmDriverExtra;
    pub dmFields: i32;
    pub short dmOrientation;
    pub short dmPaperSize;
    pub short dmPaperLength;
    pub short dmPaperWidth;
    pub short dmScale;
    pub short dmCopies;
    pub short dmDefaultSource;
    pub short dmPrintQuality;
    pub short dmColor;
    pub short dmDuplex;
    pub short dmYResolution;
    pub short dmTTOption;
    pub short dmCollate;
    [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 32)]
    pub dmFormName: String;
    pub short dmLogPixels;
    pub short dmBitsPerPel;
    pub dmPelsWidth: i32;
    pub dmPelsHeight: i32;
    pub dmDisplayFlags: i32;
    pub dmDisplayFrequency: i32;
    pub dmICMMethod: i32;
    pub dmICMIntent: i32;
    pub dmMediaType: i32;
    pub dmDitherType: i32;
    pub dmReserved1: i32;
    pub dmReserved2: i32;
    pub dmPanningWidth: i32;
    pub dmPanningHeight: i32;
  }
}
