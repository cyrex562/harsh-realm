// Decompiled with JetBrains decompiler
#![allow(non_snake_case)]
// Type: WindowsApplication1.LibIdClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// using System;
// using System.IO;
// using System.Runtime.Serialization;
// using System.Runtime.Serialization.Formatters.Binary;

// namespace WindowsApplication1
// {
// [Serializable]
// pub class LibIdClass : ISerializable
#[derive(Debug, Clone, Default)]
pub struct LibIdClass {
    pub id: i32,
    pub libSlot: i32,
}

impl LibIdClass {
    pub fn new() -> Self {
        Self {
            id: -1,
            libSlot: -1
        }
        // self.id = -1;
        // self.libSlot = -1;
    }

    // pub LibIdClass Clone()
    // {
    //   BinaryFormatter binaryFormatter = BinaryFormatter::new();
    //   MemoryStream serializationStream = MemoryStream::new();
    //   binaryFormatter.Serialize((Stream) serializationStream,  this);
    //   serializationStream.Position = 0L;
    //   return (LibIdClass) binaryFormatter.Deserialize((Stream) serializationStream);
    // }

    // pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    // {
    //   info.AddValue("id", self.id);
    //   info.AddValue("lidSlot", self.libSlot);
    // }

    // protected LibIdClass(SerializationInfo info, StreamingContext context)
    // {
    //   self.id = info.GetInt32(nameof (id));
    //   self.libSlot = info.GetInt32("lidSlot");
    // }

    pub fn SetFromAdvancedEditor(&mut self, data: DataClass, tLibSlot: i32) {
        self.id = -1;
        self.libSlot = tLibSlot;
    }
}
// }
