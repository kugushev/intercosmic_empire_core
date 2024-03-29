// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using AK.Scripts.Core.Native;
using UnityEngine;
#pragma warning restore 0105

namespace AK.Scripts.Core.Native
{
    public static partial class Interop
    {
        public const string NativeLib = "intercosmic_empire";

        static Interop()
        {
        }


        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_init_app")]
        public static extern FFIOutcome ice_init_app(ref IntPtr context, AppSettings settings);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_close_app")]
        public static extern FFIOutcome ice_close_app(ref IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_stellar_system_get_default_parameters")]
        public static extern StellarSystemParameters ice_stellar_system_get_default_parameters();

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_registry_get_planet_ratio")]
        public static extern float ice_registry_get_planet_ratio(PlanetSize size);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_get_last_exception")]
        public static extern IntPtr ice_get_last_exception(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_subscribe_logs")]
        public static extern FFIOutcome ice_subscribe_logs(IntPtr context, FFILog log_delegate);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_toggle_trace")]
        public static extern FFIOutcome ice_toggle_trace(IntPtr context, Bool enabled);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_get_current_game_variant")]
        public static extern FFIResultFFIGameVariant ice_get_current_game_variant(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_start_sandbox")]
        public static extern FFIOutcome ice_start_sandbox(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_close")]
        public static extern FFIOutcome ice_sandbox_close(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_get_battle_settings")]
        public static extern FFIResultBattleSettings ice_sandbox_get_battle_settings(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_set_battle_settings")]
        public static extern FFIOutcome ice_sandbox_set_battle_settings(IntPtr context, BattleSettings settings);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_get_stellar_system_parameters")]
        public static extern FFIResultStellarSystemParameters ice_sandbox_get_stellar_system_parameters(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_set_stellar_system_parameters")]
        public static extern FFIOutcome ice_sandbox_set_stellar_system_parameters(IntPtr context, StellarSystemParameters parameters);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_add_warpgate")]
        public static extern FFIOutcome ice_sandbox_add_warpgate(IntPtr context, Faction faction);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_clean_warpgates")]
        public static extern FFIOutcome ice_sandbox_clean_warpgates(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_start_battle")]
        public static extern FFIOutcome ice_sandbox_start_battle(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_sandbox_stop_battle")]
        public static extern FFIOutcome ice_sandbox_stop_battle(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_is_active")]
        public static extern FFIResultBool ice_battle_is_active(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_update")]
        public static extern FFIOutcome ice_battle_update(IntPtr context, float delta_time);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_get_vm")]
        public static extern FFIResultBattleViewModel ice_battle_get_vm(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_get_settings")]
        public static extern FFIResultBattleSettings ice_battle_get_settings(IntPtr context);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_get_constants")]
        public static extern Constants ice_battle_get_constants();

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_get_vm")]
        public static extern FFIResultFleetViewModel ice_battle_fleet_get_vm(IntPtr context, Faction faction);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_spawn_begin")]
        public static extern FFIResultSpawnInfo ice_battle_fleet_spawn_begin(IntPtr context, Faction faction, RouteBuildersSource builder_source, int spawner_id, SpaceshipMark mark);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_spawn_add_waypoint")]
        public static extern FFIOutcome ice_battle_fleet_spawn_add_waypoint(IntPtr context, SpawnInfo info, ref Vector3 waypoint);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_spawn_finish")]
        public static extern FFIOutcome ice_battle_fleet_spawn_finish(IntPtr context, SpawnInfo info, int finish_id);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_deck_peek_left")]
        public static extern FFIResultSpaceshipMark ice_battle_fleet_deck_peek_left(IntPtr context, Faction faction);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_deck_peek_right")]
        public static extern FFIResultSpaceshipMark ice_battle_fleet_deck_peek_right(IntPtr context, Faction faction);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_deck_pop_left")]
        public static extern FFIResultSpaceshipMark ice_battle_fleet_deck_pop_left(IntPtr context, Faction faction);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_fleet_deck_pop_right")]
        public static extern FFIResultSpaceshipMark ice_battle_fleet_deck_pop_right(IntPtr context, Faction faction);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_spaceship_get_cost")]
        public static extern FFIResultu8 ice_battle_spaceship_get_cost(IntPtr context, Faction faction, SpaceshipMark mark);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_battle_productive_can_produce")]
        public static extern FFIResultBool ice_battle_productive_can_produce(IntPtr context, int spawner_id, byte cost);

        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "ice_test_quat_look_rotation")]
        public static extern Quaternion ice_test_quat_look_rotation(Vector3 forward, Vector3 up);

    }

    public enum AiAgentType
    {
        NoOp = 0,
        RandomSpawn = 1,
    }

    public enum FFIGameVariant
    {
        None = 0,
        Sandbox = 1,
    }

    public enum FFIOutcome
    {
        Ok = 0,
        Unable = 1,
        Error = 2,
        Panic = 3,
    }

    public enum Faction
    {
        Neutral = 0,
        Enemy = 1,
        Player = 2,
        Ally = 3,
        Abandoned = 4,
    }

    public enum PlanetSize
    {
        Mercury = 0,
        Mars = 1,
        Earth = 2,
        Uranus = 3,
        Saturn = 4,
        Jupiter = 5,
    }

    public enum RouteBuildersSource
    {
        LeftHand = 0,
        RightHand = 1,
        Ai = 2,
    }

    public enum SpaceshipMark
    {
        Viper = 0,
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct AppSettings
    {
        public bool flat_mode;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct BattleSettings
    {
        public ulong seed;
        public ushort day_of_year;
        public Bool player_fleet_enabled;
        public Bool enemy_fleet_enabled;
        public AiAgentType enemy_fleet_ai;
        public Bool ally_fleet_enabled;
        public AiAgentType ally_fleet_ai;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct BattleViewModel
    {
        public IntPtr stellar_system;
        public StructVec8Faction fleets;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Constants
    {
        public float gap_between_waypoints;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultBattleSettings
    {
        public BattleSettings value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultBattleViewModel
    {
        public BattleViewModel value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultBool
    {
        public Bool value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultFFIGameVariant
    {
        public FFIGameVariant value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultFleetViewModel
    {
        public FleetViewModel value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultSpaceshipMark
    {
        public SpaceshipMark value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultSpawnInfo
    {
        public SpawnInfo value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultStellarSystemParameters
    {
        public StellarSystemParameters value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFIResultu8
    {
        public byte value;
        public FFIOutcome outcome;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FFITranslation
    {
        public Vector3 position;
        public Quaternion rotation;
        public float scale;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct FleetViewModel
    {
        public SliceSpaceshipViewModel spaceships;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Orbit
    {
        public float radius;
        public float alpha_rotation;
        public float beta_rotation;
        public int start_day;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Planet
    {
        public PlanetInfo info;
        public Vector3 position;
        public Faction faction;
        public float current_product;
        public Bool under_siege;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct PlanetId
    {
        public int x0;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct PlanetInfo
    {
        public PlanetId id;
        public Orbit orbit;
        public PlanetSize size;
        public Production production;
        public Spaceport spaceport;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Production
    {
        public float amount_per_second;
        public float max_product;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Spaceport
    {
        float orbit_radius;
        float surface_radius;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SpaceshipViewModel
    {
        public ulong id;
        public FFITranslation translation;
        public Faction faction;
        public SpaceshipMark mark;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SpawnInfo
    {
        public Faction faction;
        public RouteBuildersSource builder_source;
        public int spawner_id;
        public int builder_id;
        public SpaceshipMark spaceship_mark;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StellarSystem
    {
        public StellarSystemInfo info;
        public StructVec8Planet planets;
        public StructVec8WarpGate warpgates;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StellarSystemId
    {
        public int x0;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StellarSystemInfo
    {
        public StellarSystemId id;
        public Sun sun;
        public StellarSystemParameters parameters;
        public StructVec8PlanetInfo planets;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StellarSystemParameters
    {
        public float system_radius;
        public float min_distance_to_sun;
        public Vector3 center;
        public float sun_min_radius;
        public float sun_max_radius;
        public int min_planets;
        public int max_planets;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StructVec8Faction
    {
        Faction items0;
        Faction items1;
        Faction items2;
        Faction items3;
        Faction items4;
        Faction items5;
        Faction items6;
        Faction items7;
        byte count;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StructVec8Planet
    {
        Planet items0;
        Planet items1;
        Planet items2;
        Planet items3;
        Planet items4;
        Planet items5;
        Planet items6;
        Planet items7;
        byte count;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StructVec8PlanetInfo
    {
        PlanetInfo items0;
        PlanetInfo items1;
        PlanetInfo items2;
        PlanetInfo items3;
        PlanetInfo items4;
        PlanetInfo items5;
        PlanetInfo items6;
        PlanetInfo items7;
        byte count;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct StructVec8WarpGate
    {
        WarpGate items0;
        WarpGate items1;
        WarpGate items2;
        WarpGate items3;
        WarpGate items4;
        WarpGate items5;
        WarpGate items6;
        WarpGate items7;
        byte count;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Sun
    {
        public Vector3 position;
        public float radius;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct WarpGate
    {
        public WarpgateId id;
        public Vector3 position;
        public Faction faction;
        public Production production;
        public float current_product;
        public Spaceport spaceport;
    }

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct WarpgateId
    {
        public int x0;
    }

    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceSpaceshipViewModel
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceSpaceshipViewModel : IEnumerable<SpaceshipViewModel>
    {
        public SliceSpaceshipViewModel(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceSpaceshipViewModel(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public SpaceshipViewModel this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(SpaceshipViewModel));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<SpaceshipViewModel>(ptr);
            }
        }
        public SpaceshipViewModel[] Copied
        {
            get
            {
                var rval = new SpaceshipViewModel[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<SpaceshipViewModel> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Bool
    {
        byte value;
    }

    public partial struct Bool
    {
        public static readonly Bool True = new Bool { value =  1 };
        public static readonly Bool False = new Bool { value =  0 };
        public Bool(bool b)
        {
            value = (byte) (b ? 1 : 0);
        }
        public bool Is => value == 1;
    }


    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate byte FFILog(string log);



    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

}
